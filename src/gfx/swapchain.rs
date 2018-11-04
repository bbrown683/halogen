use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backbuffer, Backend, Device, FrameSync, SurfaceCapabilities, PresentMode, Surface,
          Swapchain, SwapchainConfig};
use hal::format::{self, ChannelType};
use hal::{image, pool};
use crate::gfx::{GfxDevice, GfxSync};

// Represents the Swapchain parameters for presenting to the screen.
pub struct GfxSwapchain<B: Backend> {
    pub current_image : u32,
    pub caps : SurfaceCapabilities,
    pub swap_config : SwapchainConfig,
    device : Rc<RefCell<GfxDevice<B>>>,
    sync : Rc<RefCell<GfxSync<B>>>,
    swapchain : Option<B::Swapchain>,
    backbuffer : Option<Backbuffer<B>>,
}

impl<B: Backend> GfxSwapchain<B> {
    // Creates a new swapchain with the given surface. This function will only need to be called once.
    // Any events that break the existing swapchain `should` call `recreate`.
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>,
               sync : Rc<RefCell<GfxSync<B>>>,
               mut surface : &mut B::Surface,
               image_count : u32) -> Result<Self,&str> {
        let (caps, formats, _present_modes) = surface.compatibility(&device.borrow().physical_device);
        if !caps.image_count.contains(&image_count) {
            return Err("image_count parameter was not within valid boundaries.");
        }

        let format = formats
            .map_or(format::Format::Rgba8Srgb, |formats| {
                formats
                    .iter()
                    .find(|format| format.base_format().1 == ChannelType::Srgb)
                    .map(|format| *format)
                    .unwrap_or(formats[0])
            });
        println!("{:?}", caps);
        let extent = caps.current_extent.unwrap().to_extent();
        let swap_config = SwapchainConfig::new(
            extent.width,
            extent.height,
            format::Format::Rgba8Unorm,
            image_count)
            .with_mode(PresentMode::Fifo); // Vulkan spec guarantee's this mode.
        println!("{:?}", swap_config);
        let (swapchain, backbuffer) = device.borrow().logical_device
            .create_swapchain(&mut surface, swap_config.clone(), None)
            .expect("Failed to create swapchain.");
        Ok(Self { current_image: 0, caps, swap_config, device, sync,
            swapchain: Some(swapchain), backbuffer: Some(backbuffer) })
    }

    pub fn recreate(self) {
        unimplemented!()
    }

    pub fn prepare_frame(mut self) {
        let image = self.swapchain.as_mut().unwrap()
            .acquire_image(u64::max_value(), FrameSync::Fence(self.sync.borrow().fence.as_ref().unwrap()))
            .expect("Failed to acquire swapchain image.");
    }

    pub fn present_frame(&mut self) {
//        &self.swapchain.as_mut().unwrap().present(self.device.borrow().queue_group.queues[0], self.current_image)
    }
}

impl<B: Backend> Drop for GfxSwapchain<B> {
    fn drop(&mut self) {
        &self.device.borrow().logical_device.destroy_swapchain(self.swapchain.take().unwrap());
        debug_assert!(self.swapchain.is_none());
    }
}