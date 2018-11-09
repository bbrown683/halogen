use hal::{Backend, Capability, Device, QueueGroup };
use hal::queue::{Submission, Supports};
use crate::gfx::CmdBuffer;

pub struct Queue<B: Backend, C: Capability> {
    queue_group : QueueGroup<B, C>,
}

impl<B: Backend, C: Capability> Drop for Queue<B, C> {
    fn drop(&mut self) {
        // Ensure all queues in group are complete before continuing with drop.
        for queue in &self.queue_group.queues {
            queue.wait_idle().expect("Failed to wait on queue to become idle.");
        }
    }
}

impl<B: Backend, C: Capability> Queue<B, C> {
    pub fn new(queue_group : QueueGroup<B, C>) -> Self {
        Self { queue_group }
    }

    /// Submits the submittable to the specified queue.
    pub fn submit<S>(mut self, cmd_buffer : &CmdBuffer<B, C>, signal_semaphores : &Vec<B::Semaphore>) where S: Supports<C> {
        /*
        let submission = Submission::new()
            .signal(signal_semaphores)
            .submit(Some(cmd_buffer.get_submittable()));
        self.queue_group.queues[0].submit(submission, cmd_buffer.get_fence());
        */
    }

    /// Returns a reference to the queue group for `Capability` operations.
    pub fn get_queue_group(&self) -> &QueueGroup<B, C> {
        &self.queue_group
    }
}