use std::cell::RefCell;
use std::rc::Rc;
use nalgebra::Vector3;
use crate::gfx::{GfxBackend, GfxBackendType, GfxDevice, GfxSwapchain,
                 GfxSync, GfxRenderable, GfxVertex};

pub struct RenderSystem {
    backend : GfxBackend,
    device : Option<Rc<RefCell<GfxDevice<GfxBackendType>>>>,
    sync : Option<Rc<RefCell<GfxSync<GfxBackendType>>>>,
    swapchain : Option<GfxSwapchain<GfxBackendType>>,
    renderable : Option<GfxRenderable<GfxBackendType>>,
}

impl Drop for RenderSystem {
    fn drop(&mut self) {
        self.renderable.take();
        debug_assert!(self.renderable.is_none());
        self.swapchain.take();
        debug_assert!(self.swapchain.is_none());
        self.sync.take();
        debug_assert!(self.sync.is_none());
        self.device.take();
        debug_assert!(self.device.is_none());
    }
}

impl RenderSystem {
    pub fn new(window : &winit::Window) -> Self {
        let mut backend = GfxBackend::new(window);
        let device = Some(Rc::new(RefCell::new(GfxDevice::new(
            backend.adapters.remove(0),
            &backend.surface
        ))));

        // Initialize syncronization primitives.
        let sync = Some(Rc::new(RefCell::new(GfxSync::new(
            Rc::clone(&device.clone().unwrap()),
            2))));

        // Create initial swapchain for rendering.
        let swapchain = GfxSwapchain::new(
            Rc::clone(&device.clone().unwrap()),
            Rc::clone(&sync.clone().unwrap()),
            &mut backend.surface, 2).ok();

        // Vertices for our triangle.
        let mut vertices = Vec::<GfxVertex>::new();
        vertices.push(GfxVertex { position : Vector3::new(-1.0, -1.0, 0.0)});
        vertices.push( GfxVertex { position : Vector3::new(1.0, -1.0, 0.0)});
        vertices.push( GfxVertex { position : Vector3::new(0.0, 1.0, 0.0)});

        let mut indices = Vec::<u16>::new();
        indices.push(0);
        indices.push(1);
        indices.push(2);

        // Render state associated with our triangle.
        let renderable = GfxRenderable::new(
            Rc::clone(&device.clone().unwrap()),
            vertices,
            indices,
            include_bytes!("../shaders/default.vert.spv").to_vec(),
            include_bytes!("../shaders/default.frag.spv").to_vec());
        Self { backend, device, sync, swapchain, renderable : Some(renderable) }
    }
}
