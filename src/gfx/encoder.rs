use std::cell::RefCell;
use std::rc::Rc;
use hal::pso;
use hal::{Backend};
use hal::command::{ClearValue, ClearColor};
use crate::gfx::{GfxDevice, GfxRenderable};

/// Records rendering state into a command buffer. The encoder can be reused for a `GfxRenderable`.
pub struct GfxEncoder<B: Backend> {
    device : Rc<RefCell<GfxDevice<B>>>,
    renderable : Option<GfxRenderable<B>>,
    viewport : pso::Viewport,
}

impl<B: Backend> Drop for GfxEncoder<B> {
    fn drop(&mut self) {
        self.renderable.take();
        debug_assert!(self.renderable.is_none());
    }
}

impl<B: Backend> GfxEncoder<B> {
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>,
               renderable :GfxRenderable<B>) -> Self {
        let viewport = pso::Viewport {
            rect: pso::Rect {
                x: 0,
                y: 0,
                w: 1024 as _,
                h: 768 as _,
            },
            depth: 0.0..1.0,
        };
        Self { device, renderable: Some(renderable), viewport }
    }

    /*
    pub fn get_submission(self, mut cmd_buffer : B::CommandBuffer) {
        cmd_buffer.set_viewports(0, &[self.viewport.clone()]);
        cmd_buffer.set_scissors(0, &[self.viewport.rect]);
        cmd_buffer.bind_graphics_pipeline(self.renderable.as_ref().unwrap().pipeline.as_ref().unwrap());
        cmd_buffer.bind_vertex_buffers(0, Some((&self.renderable.as_ref().unwrap().vertex_buffer.as_ref().unwrap(), 0)));
        let mut encoder = cmd_buffer.begin_render_pass_inline(
            self.renderable.as_ref().unwrap().render_pass.as_ref().unwrap(),
            framebuffer,
            self.viewport.rect,
            &[ClearValue::Color(ClearColor::Float([1.0, 1.0, 0.0, 1.0]))],
        );
        encoder.draw(0..6, 0..1);
        cmd_buffer.finish();
    }
    */
}