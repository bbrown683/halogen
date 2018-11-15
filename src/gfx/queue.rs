use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, Capability, Compute, Device, Graphics, QueueGroup, Transfer};
use hal::queue::{Submission, Supports};
use crate::gfx::CmdBuffer;

pub struct QueueSet<B: Backend> {
    compute : Rc<RefCell<Queue<B, Compute>>>,
    graphics : Rc<RefCell<Queue<B, Graphics>>>,
    transfer : Rc<RefCell<Queue<B, Transfer>>>,
}

impl<B: Backend> QueueSet<B> {
    pub fn new(compute : Queue<B, Compute>,
               graphics : Queue<B, Graphics>,
               transfer : Queue<B, Transfer>) -> Self {
        Self {
            compute: Rc::new(RefCell::new(compute)),
            graphics: Rc::new(RefCell::new(graphics)),
            transfer: Rc::new(RefCell::new(transfer))
        }
    }

    pub fn get_compute_queue(&self) -> &Rc<RefCell<Queue<B, Compute>>> {
        &self.compute
    }

    pub fn get_graphics_queue(&self) -> &Rc<RefCell<Queue<B, Graphics>>> {
        &self.graphics
    }

    pub fn get_transfer_queue(&self) -> &Rc<RefCell<Queue<B, Transfer>>> {
        &self.transfer
    }

    pub fn get_compute_queue_mut(&mut self) -> &mut Rc<RefCell<Queue<B, Compute>>> {
        &mut self.compute
    }

    pub fn get_graphics_queue_mut(&mut self) -> &mut Rc<RefCell<Queue<B, Graphics>>> {
        &mut self.graphics
    }

    pub fn get_transfer_queue_mut(&mut self) -> &mut Rc<RefCell<Queue<B, Transfer>>> {
        &mut self.transfer
    }
}

pub struct Queue<B: Backend, C: Capability> {
    queue_group : QueueGroup<B, C>,
}

impl<B: Backend, C: Capability> Drop for Queue<B, C> {
    fn drop(&mut self) {
        // Ensure all queues in group are complete before continuing with drop.
        for queue in &self.queue_group.queues {
            queue.wait_idle().expect("Failed to wait on queue to become idle.");
        }
        info!("Dropped Queue")
    }
}

impl<B: Backend, C: Capability> Queue<B, C> {
    pub fn new(queue_group : QueueGroup<B, C>) -> Self {
        Self { queue_group }
    }

    /// Submits the submittable to the specified queue.
    pub fn submit<S>(mut self, cmd_buffer : &CmdBuffer<B>, signal_semaphores : &Vec<B::Semaphore>) where S: Supports<C> {
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

    /// Returns a mutable reference to the queue group for `Capability` operations.
    pub fn get_queue_group_mut(&mut self) -> &mut QueueGroup<B, C> {
        &mut self.queue_group
    }
}