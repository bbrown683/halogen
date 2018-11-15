use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, Capability, Device as LogicalDevice};
use hal::command::{RawCommandBuffer, RawLevel};
use hal::pool::{CommandPoolCreateFlags, RawCommandPool};
use crate::gfx::{Device, Queue};

pub struct PoolSet<B: Backend> {
    compute : Rc<RefCell<CmdPool<B>>>,
    graphics : Rc<RefCell<CmdPool<B>>>,
    transfer  : Rc<RefCell<CmdPool<B>>>
}

impl<B: Backend> PoolSet<B> {
    pub fn new(compute : CmdPool<B>,
               graphics : CmdPool<B>,
               transfer  : CmdPool<B>) -> Self {
        Self {
            compute: Rc::new(RefCell::new(compute)),
            graphics: Rc::new(RefCell::new(graphics)),
            transfer: Rc::new(RefCell::new(transfer)),
        }
    }

    pub fn get_compute_pool(&self) -> &Rc<RefCell<CmdPool<B>>> {
        &self.compute
    }

    pub fn get_graphics_pool(&self) -> &Rc<RefCell<CmdPool<B>>> {
        &self.graphics
    }

    pub fn get_transfer_pool(&self) -> &Rc<RefCell<CmdPool<B>>> {
        &self.transfer
    }
}

/// A recorder for graphics, compute, or transfer operations.
pub struct CmdBuffer<B: Backend> {
    device : Rc<RefCell<Device<B>>>,
    cmd_pool : Rc<RefCell<CmdPool<B>>>,
    cmd_buffer : Option<B::CommandBuffer>,
    fence : Option<B::Fence>,
}

impl<B: Backend> Drop for CmdBuffer<B> {
    fn drop(&mut self) {
        unsafe {
            let cmd_buffers = vec![self.cmd_buffer.take().unwrap()];
            &self.cmd_pool.borrow_mut().get_cmd_pool().free(cmd_buffers);
        }
        debug_assert!(self.cmd_buffer.is_none());
        &self.device.borrow().get_logical_device().destroy_fence(self.fence.take().unwrap());
        debug_assert!(self.fence.is_none());
        info!("Dropped CmdBuffer")
    }
}

impl<B: Backend> CmdBuffer<B> {
    pub fn new(device : Rc<RefCell<Device<B>>>, cmd_pool : Rc<RefCell<CmdPool<B>>>) -> Self {
        let cmd_buffer = Some(cmd_pool.borrow_mut().get_cmd_pool().allocate(1, RawLevel::Primary).remove(0));
        let fence = Some(device.borrow().get_logical_device().create_fence(true).expect("Failed to create fence."));
        Self { device, cmd_pool, cmd_buffer, fence }
    }

    pub fn begin_pass(&mut self) {  }

    pub fn end_pass(&mut self) {  }

    pub fn get_fence(&self) -> &Option<B::Fence> {
        &self.fence
    }
}

/// Allocates the command buffers into memory for reuse.
pub struct CmdPool<B: Backend> {
    device : Rc<RefCell<Device<B>>>,
    cmd_pool : Option<B::CommandPool>,
}

impl<B: Backend> Drop for CmdPool<B> {
    fn drop(&mut self) {
        &self.device.borrow().get_logical_device().destroy_command_pool(self.cmd_pool.take().unwrap());
        debug_assert!(self.cmd_pool.is_none());
        info!("Dropped CmdPool")
    }
}

impl<B: Backend> CmdPool<B> {
    pub fn new<C: Capability>(device : Rc<RefCell<Device<B>>>, queue : &mut Queue<B, C>) -> Self {
        let cmd_pool = Some(device
            .borrow()
            .get_logical_device()
            .create_command_pool(queue.get_queue_group_mut().family(),
                                       CommandPoolCreateFlags::RESET_INDIVIDUAL)
            .expect("Failed to create command pool."));
        Self { device, cmd_pool }
    }

    pub fn reset(&mut self) {
        self.cmd_pool.as_mut().unwrap().reset();
    }

    pub fn get_cmd_pool(&mut self) -> &mut B::CommandPool {
        self.cmd_pool.as_mut().unwrap()
    }
}