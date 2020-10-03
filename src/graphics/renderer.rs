use std::{cell::RefCell, iter, rc::Rc};
use std::sync::{Arc, Mutex};
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::window::Window;
use super::{ColoredMaterial, CmdBuffer, CmdPool, CmdState, Device, Framebuffer, FramebufferBuilder, Instance, Pipeline,
            PipelineBuilder, RenderPass, RenderPassBuilder, Swapchain, Queue};
use crate::util::CapturedEvent;

/// The highest level of the graphics module, the `Renderer` manages all render state.
pub struct Renderer {
    instance : Option<Rc<RefCell<Instance>>>,
    device : Option<Rc<RefCell<Device>>>,
    compute_queue : Option<Rc<RefCell<Queue>>>,
    graphics_queue : Option<Rc<RefCell<Queue>>>,
    transfer_queue : Option<Rc<RefCell<Queue>>>,
    swapchain : Option<Swapchain>,
    render_pass: Option<Rc<RefCell<RenderPass>>>,
    colored_graphics_pipeline : Option<Pipeline>,
    framebuffers : Option<Vec<Framebuffer>>,
    graphics_pool : Option<Rc<RefCell<CmdPool>>>,
    graphics_buffer : Option<CmdBuffer>,
    colored_material : Option<ColoredMaterial>,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.colored_material.take();
        debug_assert!(self.colored_material.is_none());
        self.graphics_buffer.take();
        debug_assert!(self.graphics_buffer.is_none());
        self.graphics_pool.take();
        debug_assert!(self.graphics_pool.is_none());
        self.framebuffers.take();
        debug_assert!(self.framebuffers.is_none());
        self.colored_graphics_pipeline.take();
        debug_assert!(self.colored_graphics_pipeline.is_none());
        self.render_pass.take();
        debug_assert!(self.render_pass.is_none());
        self.swapchain.take();
        debug_assert!(self.swapchain.is_none());
        self.compute_queue.take();
        debug_assert!(self.compute_queue.is_none());
        self.graphics_queue.take();
        debug_assert!(self.graphics_queue.is_none());
        self.transfer_queue.take();
        debug_assert!(self.transfer_queue.is_none());
        self.device.take();
        debug_assert!(self.device.is_none());
        self.instance.take();
        debug_assert!(self.instance.is_none());
        info!("Dropped Renderer")
    }
}

impl CapturedEvent for Renderer {
    /// When this event is captured, the swapchain is recreated, and regenerates all framebuffers from the swapchain images.
    fn on_resize(&mut self, _size : LogicalSize<f32>) {
        self.swapchain.as_mut().unwrap().recreate();
        self.framebuffers.as_mut().unwrap().clear();
        for image in self.swapchain.as_ref().unwrap().images() {
            self.framebuffers.as_mut().unwrap().push(FramebufferBuilder::new(
                Rc::clone(&self.device.clone().unwrap()),
                Rc::clone(&self.render_pass.clone().unwrap()),
                image,
                self.swapchain.as_ref().unwrap().surface_format().format,
                self.swapchain.as_ref().unwrap().capabilities().current_extent)
                .build());
        }
    }
}

impl Renderer {
    /// Initializes the renderer for the specified window.
    pub fn new(window : &Window) -> Self {
        info!("Initializing Renderer.");
        // TODO: Properly handle errors here and present them to the output.

        let instance = Rc::new(RefCell::new(Instance::new()
            .ok()
            .unwrap()));

        let device = Rc::new(RefCell::new(Device::new(&instance.borrow())
            .ok()
            .unwrap()));

        // Create our queues.
        let compute_queue = Rc::new(RefCell::new(Queue::new(
            Rc::clone(&device),
            device.borrow().compute_queue_index())));
        let graphics_queue = Rc::new(RefCell::new(Queue::new(
            Rc::clone(&device),
            device.borrow().graphics_queue_index())));
        let transfer_queue = Rc::new(RefCell::new(Queue::new(
            Rc::clone(&device),
            device.borrow().transfer_queue_index())));

        // Create the swapchain.
        let swapchain = Swapchain::new(
            Rc::clone(&instance),
            Rc::clone(&device),
            Rc::clone(&graphics_queue),
            window,
            2).ok()
            .unwrap();

        let render_pass = Rc::new(RefCell::new(RenderPassBuilder::new(
            Rc::clone(&device))
            .add_color_attachment(swapchain.surface_format().format)
            .build()));

        let colored_material = ColoredMaterial::new(Rc::clone(&device));

        let colored_graphics_pipeline = PipelineBuilder::new(Rc::clone(&device))
            .build_graphics(&render_pass.borrow(), &colored_material, swapchain.capabilities().current_extent);

        // Grab the swapchain images to create the framebuffers.
        let mut framebuffers = Vec::<Framebuffer>::new();
        for image in swapchain.images() {
            framebuffers.push(FramebufferBuilder::new(
                Rc::clone(&device),
                Rc::clone(&render_pass),
                image,
                swapchain.surface_format().format,
                swapchain.capabilities().current_extent
            ).build());
        }

        let graphics_pool = Rc::new(RefCell::new(CmdPool::new(
            Rc::clone(&device),
            &graphics_queue.borrow())));

        let graphics_buffer = CmdBuffer::new(
            Rc::clone(&device),
            Rc::clone(&graphics_pool));

        info!("Renderer has been initialized.");
        Self {
            instance: Some(instance),
            device: Some(device),
            compute_queue: Some(compute_queue),
            graphics_queue: Some(graphics_queue),
            transfer_queue: Some(transfer_queue),
            swapchain: Some(swapchain),
            render_pass: Some(render_pass),
            colored_graphics_pipeline : Some(colored_graphics_pipeline),
            framebuffers: Some(framebuffers),
            graphics_pool: Some(graphics_pool),
            graphics_buffer: Some(graphics_buffer),
            colored_material: Some(colored_material)
        }
    }

    pub fn draw_frame(&mut self) {
        let next_image = &self.swapchain.as_mut().unwrap().acquire_next_image();
        let cmd_state = CmdState {
            format: self.swapchain.as_ref().unwrap().surface_format().format,
            extent: self.swapchain.as_ref().unwrap().capabilities().current_extent
        };

        self.graphics_buffer
            .as_mut()
            .unwrap()
            .record_graphics(
                cmd_state,
                &self.render_pass.as_ref().unwrap().borrow(),
                self.framebuffers.as_ref().unwrap().get(next_image.clone() as usize).unwrap(),
                self.colored_graphics_pipeline.as_ref().unwrap());

        // Queue needs to submit our draw calls, but has to wait for the image to be acquired.
        self.graphics_queue
            .as_ref()
            .unwrap()
            .borrow()
            .submit(self.graphics_buffer.as_ref().unwrap(),
                    Some(self.swapchain.as_ref().unwrap().current_acquire_semaphore()),
                    Some(self.swapchain.as_ref().unwrap().current_acquire_fence()));
        self.swapchain.as_ref().unwrap().present();
    }
}
