mod cmd_buffer;
mod cmd_pool;
mod device;
mod framebuffer;
mod material;
mod pass;
mod pipeline;
mod queue;
mod swapchain;
mod renderer;

pub use self::cmd_buffer::CmdBuffer;
pub use self::cmd_pool::CmdPool;
pub use self::device::Device;
pub use self::framebuffer::GfxFramebuffer;
pub use self::material::{Material, MaterialBuilder};
pub use self::pass::{RenderPass, RenderPassBuilder};
pub use self::pipeline::{Pipeline, PipelineBuilder};
pub use self::queue::Queue;
pub use self::renderer::Renderer;
pub use self::swapchain::Swapchain;

use hal::{Adapter, Backend as BackendTrait, Instance};
use nalgebra::Vector3;

/// Core backend type for gfx based on the crate.
pub type BackendType = back::Backend;

/// Vertex structure used by the engine.
#[derive(Clone, Copy)]
pub struct GfxVertex {
    /// 3-Dimensional coordinates in space denoted by x, y, and z.
    pub position : Vector3<f32>,
//    pub color : Vector4<f32>,
}

/// Manages the required components to initialize the gfx library.
pub struct Backend {
    _instance : back::Instance,
    surface : <BackendType as BackendTrait>::Surface,
    adapters : Vec<Adapter<BackendType>>,
}

impl Backend {
    pub fn new(window : &winit::Window) -> Self {
        let instance = back::Instance::create("halogen", 1);
        let surface = instance.create_surface(window);
        let adapters = instance.enumerate_adapters();
        Self { _instance: instance, surface, adapters }
    }

    pub fn get_surface(&mut self) -> &mut <BackendType as BackendTrait>::Surface {
        &mut self.surface
    }

    pub fn get_primary_adapter(&mut self) -> Adapter<BackendType> {
        self.adapters.remove(0)
    }
}

