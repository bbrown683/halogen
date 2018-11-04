mod device;
mod encoder;
mod renderable;
mod swapchain;
mod sync;

pub use self::device::GfxDevice;
pub use self::encoder::GfxEncoder;
pub use self::renderable::{GfxRenderable, GfxVertex};
pub use self::swapchain::GfxSwapchain;
pub use self::sync::GfxSync;

use hal::{Adapter, Backend, Instance};

// Core backend type for gfx based on the crate.
pub type GfxBackendType = back::Backend;

pub struct GfxBackend {
    pub instance : back::Instance,
    pub surface : <GfxBackendType as Backend>::Surface,
    pub adapters : Vec<Adapter<GfxBackendType>>,
}

impl GfxBackend {
    pub fn new(window : &winit::Window) -> Self {
        let instance = back::Instance::create("halogen", 1);
        let surface = instance.create_surface(window);
        let adapters = instance.enumerate_adapters();
        Self { instance, surface, adapters }
    }
}