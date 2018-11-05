use std::cell::RefCell;
use std::rc::Rc;
use hal::{format, image};
use hal::{Backbuffer, Backend, Device, FrameSync, PresentMode, Surface, SurfaceCapabilities,
          SwapchainConfig};
use hal::command::{ClearValue, ClearColor};
use hal::queue::Submission;
use hal::pso::{Rect, Viewport};
use crate::gfx::{GfxDevice, GfxSync, GfxRenderable};

/// Controls the presentation to a surface.
pub struct GfxSwapchain<B: Backend> {
    device : Rc<RefCell<GfxDevice<B>>>,
    sync : Rc<RefCell<GfxSync<B>>>,
    current_image : u32,
    caps : SurfaceCapabilities,
    swap_config : SwapchainConfig,
    viewport : Viewport,
    swapchain : Option<B::Swapchain>,
    frame_images: Option<Vec<(B::Image, B::ImageView)>>,
    framebuffers: Option<Vec<B::Framebuffer>>
}

impl<B: Backend> Drop for GfxSwapchain<B> {
    fn drop(&mut self) {
        for framebuffer in self.framebuffers.take().unwrap() {
            &self.device.borrow().get_logical_device().borrow().destroy_framebuffer(framebuffer);
        }
        debug_assert!(self.framebuffers.is_none());
        for (_, rtv) in self.frame_images.take().unwrap() {
            &self.device.borrow().get_logical_device().borrow().destroy_image_view(rtv);
        }
        debug_assert!(self.frame_images.is_none());
        &self.device.borrow().get_logical_device().borrow().destroy_swapchain(self.swapchain.take().unwrap());
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
        let (caps, _formats, _present_modes) = surface.compatibility(&device.borrow().get_physical_device().borrow());
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

        let viewport = Viewport {
            rect: Rect {
                x: 0,
                y: 0,
                w: extent.width as _,
                h: extent.height as _,
            },
            depth: 0.0..1.0,
        };

        let (swapchain, backbuffer) = device
            .borrow()
            .get_logical_device()
            .borrow()
            .create_swapchain(&mut surface, swap_config.clone(), None)
            .expect("Failed to create swapchain.");

        let (frame_images, framebuffers) = match backbuffer {
            Backbuffer::Images(images) => {
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

                        let rtv = device
                            .borrow()
                            .get_logical_device()
                            .borrow()
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
                        device
                            .borrow()
                            .get_logical_device()
                            .borrow()
                            .create_framebuffer(device.borrow().get_render_pass().borrow().as_ref().unwrap(),
                                                Some(rtv), extent)
                            .unwrap()
                    })
                    .collect();
                (pairs, fbos)
            }
            Backbuffer::Framebuffer(fbo) => (Vec::new(), vec![fbo]),
        };

        Ok(Self { device, sync, current_image: 0, caps, swap_config, viewport, swapchain: Some(swapchain),
            frame_images: Some(frame_images), framebuffers: Some(framebuffers) })
    }

    pub fn get_current_image(&self) -> u32 {
        self.current_image.clone()
    }

    pub fn get_surface_capabilities(&self) -> SurfaceCapabilities {
        self.caps.clone()
    }

    pub fn get_swapchain_config(&self) -> SwapchainConfig {
        self.swap_config.clone()
    }

    pub fn get_viewport(&self) -> Viewport {
        self.viewport.clone()
    }

    pub fn get_swapchain(&self) -> Rc<RefCell<&Option<B::Swapchain>>> {
        Rc::from(RefCell::from(&self.swapchain))
    }

    pub fn get_frame_images(&self) -> Rc<RefCell<&Option<Vec<(B::Image, B::ImageView)>>>> {
        Rc::from(RefCell::from(&self.frame_images))
    }

    pub fn get_framebuffers(&self) -> Rc<RefCell<&Option<Vec<B::Framebuffer>>>> {
        Rc::from(RefCell::from(&self.framebuffers))
    }

    /*
pub fn submit_renderable(self, renderable : &GfxRenderable<B>) {
    let submit = {
        let mut cmd_buffer = self.device.borrow().command_pool.as_ref().unwrap().acquire_command_buffer(false);
        {
            cmd_buffer.set_viewports(0, &[self.viewport.clone()]);
            cmd_buffer.set_scissors(0, &[self.viewport.rect]);
            cmd_buffer.bind_graphics_pipeline(renderable.pipeline.as_ref().unwrap());
            cmd_buffer.bind_vertex_buffers(0, Some((renderable.vertex_buffer.as_ref().unwrap(), 0)));
            {
                let mut encoder = cmd_buffer.begin_render_pass_inline(
                    self.device.borrow().render_pass.as_ref().unwrap(),
                    self.framebuffers.as_ref().unwrap().as_slice().get(self.current_image).unwrap(),
                    self.viewport.rect,
                    &[ClearValue::Color(ClearColor::Float([1.0, 1.0, 0.0, 1.0]))],
                );
                encoder.draw(0..3, 0..1);
            }
        }
        cmd_buffer.finish()
    };
}


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