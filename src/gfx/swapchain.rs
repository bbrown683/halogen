use std::cell::RefCell;
use std::rc::Rc;
use hal::{format, image};
use hal::{Backbuffer, Backend, Device, SurfaceCapabilities, PresentMode, Surface, SwapchainConfig};
use crate::gfx::{GfxDevice, GfxSync};

/// Controls the presentation to a surface.
pub struct GfxSwapchain<B: Backend> {
    device : Rc<RefCell<GfxDevice<B>>>,
    sync : Rc<RefCell<GfxSync<B>>>,
    pub current_image : u32,
    pub caps : SurfaceCapabilities,
    pub swap_config : SwapchainConfig,
    pub swapchain : Option<B::Swapchain>,
    pub frame_images: Option<Vec<(B::Image, B::ImageView)>>,
    pub framebuffers: Option<Vec<B::Framebuffer>>,
}

impl<B: Backend> Drop for GfxSwapchain<B> {
    fn drop(&mut self) {
        for framebuffer in self.framebuffers.take().unwrap() {
            &self.device.borrow().logical_device.destroy_framebuffer(framebuffer);
        }
        debug_assert!(self.framebuffers.is_none());
        for (_, rtv) in self.frame_images.take().unwrap() {
            &self.device.borrow().logical_device.destroy_image_view(rtv);
        }
        debug_assert!(self.frame_images.is_none());
        &self.device.borrow().logical_device.destroy_swapchain(self.swapchain.take().unwrap());
        debug_assert!(self.swapchain.is_none());
    }
}

impl<B: Backend> GfxSwapchain<B> {
    /// Creates a new swapchain with the given surface. This function will only need to be called once.
    /// Any events that break the existing swapchain `should` call `recreate`.
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>,
               sync : Rc<RefCell<GfxSync<B>>>,
               mut surface : &mut B::Surface,
               image_count : u32) -> Result<Self,&str> {
        let (caps, _formats, _present_modes) = surface.compatibility(&device.borrow().physical_device);
        if !caps.image_count.contains(&image_count) {
            return Err("image_count parameter was not within valid boundaries.");
        }

        let extent = caps.current_extent.unwrap().to_extent();
        let swap_config = SwapchainConfig::new(
            extent.width,
            extent.height,
            format::Format::Rgba8Unorm,
            image_count)
            .with_mode(PresentMode::Fifo); // Vulkan spec guarantee's this mode.
        let (swapchain, backbuffer) = device.borrow().logical_device
            .create_swapchain(&mut surface, swap_config.clone(), None)
            .expect("Failed to create swapchain.");

        let (frame_images, framebuffers) = match backbuffer {
            Backbuffer::Images(images) => {
                let device_borrow = device.borrow();

                let extent = image::Extent {
                    width: extent.width as _,
                    height: extent.height as _,
                    depth: 1,
                };
                let pairs = images
                    .into_iter()
                    .map(|image| {
                        let color_range = image::SubresourceRange {
                            aspects: format::Aspects::COLOR,
                            levels: 0..1,
                            layers: 0..1,
                        };

                        let rtv = device_borrow.logical_device
                            .create_image_view(
                                &image,
                                image::ViewKind::D2,
                                format::Format::Rgba8Unorm,
                                format::Swizzle::NO,
                                color_range,
                            )
                            .unwrap();
                        (image, rtv)
                    })
                    .collect::<Vec<_>>();
                let fbos = pairs
                    .iter()
                    .map(|&(_, ref rtv)| {
                        device_borrow.logical_device
                            .create_framebuffer(device_borrow.render_pass.as_ref().unwrap(), Some(rtv), extent)
                            .unwrap()
                    })
                    .collect();
                (pairs, fbos)
            }
            Backbuffer::Framebuffer(fbo) => (Vec::new(), vec![fbo]),
        };

        Ok(Self { current_image: 0, caps, swap_config, device, sync,
            swapchain: Some(swapchain), frame_images: Some(frame_images), framebuffers: Some(framebuffers) })
    }

    /*
    pub fn prepare_frame(mut self) {
        let _image = self.swapchain.as_mut().unwrap()
            .acquire_image(u64::max_value(), FrameSync::Fence(self.sync.borrow().fence.as_ref().unwrap()))
            .expect("Failed to acquire swapchain image.");
    }

    pub fn present_frame(&mut self) {
        &self.swapchain.as_mut().unwrap().present(self.device.borrow().queue_group.queues[0], self.current_image)
    }
    */
}