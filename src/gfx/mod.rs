/// Module layout is inspired by UE4's VulkanRHI, but will be a very watered down variant.

mod cmd_buffer;
mod cmd_pool;
mod device;
mod framebuffer;
mod pass;
mod swapchain;

pub use self::cmd_buffer::GfxCmdBuffer;
pub use self::cmd_pool::GfxCmdPool;
pub use self::device::GfxDevice;
pub use self::framebuffer::GfxFramebuffer;
pub use self::pass::GfxRenderPass;
pub use self::swapchain::GfxSwapchain;

use hal::{Adapter, Backend, Instance};
use nalgebra::Vector3;

/// Core backend type for gfx based on the crate.
pub type GfxBackendType = back::Backend;

/// Vertex structure used by the engine.
#[derive(Clone, Copy)]
pub struct GfxVertex {
    /// 3-Dimensional coordinates in space denoted by x, y, and z.
    pub position : Vector3<f32>,
//    pub color : Vector4<f32>,
}

/// Manages the required components to initialize the gfx library.
pub struct GfxBackend {
    instance : back::Instance,
    surface : <GfxBackendType as Backend>::Surface,
    adapters : Vec<Adapter<GfxBackendType>>,
}

impl GfxBackend {
    pub fn new(window : &winit::Window) -> Self {
        let instance = back::Instance::create("halogen", 1);
        let surface = instance.create_surface(window);
        let adapters = instance.enumerate_adapters();
        Self { instance, surface, adapters }
    }

    pub fn get_surface(&mut self) -> Option<&mut <GfxBackendType as Backend>::Surface> {
        Some(&mut self.surface)
    }

    pub fn get_primary_adapter(&mut self) -> Adapter<GfxBackendType> {
        self.adapters.remove(0)
    }
}

