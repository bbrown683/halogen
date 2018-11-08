use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, Capability, Device, QueueGroup };
use hal::queue::Submission;
use crate::gfx::{GfxCmdBuffer};

pub struct GfxQueue<B: Backend, C: Capability> {
    queue_group : QueueGroup<B, C>,
}

impl<B: Backend, C: Capability> Drop for GfxQueue<B, C> {
    fn drop(&mut self) {
        // Ensure all queues in group are complete before continuing with drop.
        for queue in &self.queue_group.queues {
            queue.wait_idle().expect("Failed to wait on queue to become idle.");
        }
    }
}

impl<B: Backend, C: Capability> GfxQueue<B, C> {
    pub fn new(queue_group : QueueGroup<B, C>) -> Self {
        Self { queue_group }
    }

    pub fn submit(cmd_buffer : Rc<RefCell<GfxCmdBuffer<B, C>>>) {
        unimplemented!()
    }

    /// Returns a reference to the queue group for `Capability` operations.
    pub fn get_queue_group(&self) -> &QueueGroup<B, C> {
        &self.queue_group
    }
}