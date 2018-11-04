use std::cell::RefCell;
use std::rc::Rc;
use hal::{Adapter, Backend, Instance, Surface };
use nalgebra::Vector3;
use crate::gfx::{GfxBackend, GfxBackendType, GfxDevice, GfxSwapchain, GfxSync, encoder, Vertex };

pub struct RenderSystem {
    backend : GfxBackend,
    device : Option<Rc<RefCell<GfxDevice<GfxBackendType>>>>,
    sync : Option<Rc<RefCell<GfxSync<GfxBackendType>>>>,
    swapchain : Option<GfxSwapchain<GfxBackendType>>,
    encoder : Option<encoder::GfxEncoder<GfxBackendType>>,
}

impl Drop for RenderSystem {
    fn drop(&mut self) {
        self.encoder.take();
        debug_assert!(self.encoder.is_none());
        self.swapchain.take();
        debug_assert!(self.swapchain.is_none());
        self.sync.take();
        debug_assert!(  self.sync.is_none());
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

        let mut vertices : Vec<Vertex> = Vec::new();
        vertices.push(Vertex { position : Vector3::new(-1.0, -1.0, 0.0)});
        vertices.push( Vertex { position : Vector3::new(1.0, -1.0, 0.0)});
        vertices.push( Vertex { position : Vector3::new(0.0, 1.0, 0.0)});

        let encoder = Some(encoder::EncoderBuilder::new(Rc::clone(&device.clone().unwrap()))
            .with_vertex_shader(include_bytes!("../shaders/default.vert.spv").to_vec())
            .with_fragment_shader(include_bytes!("../shaders/default.frag.spv").to_vec())
            .with_vertices(vertices)
            .build());
        Self { backend, device, sync, swapchain, encoder }
    }

    pub fn draw_scene(self) {

    }

    pub fn on_resize(self, width : u32, height : u32) {
        unimplemented!()
    }
}
