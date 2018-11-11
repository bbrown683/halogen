use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, Capability, CommandPool, Device as LogicalDevice};
use hal::command::{CommandBuffer, Primary, MultiShot, Submittable};
use hal::pool::{CommandPoolCreateFlags};
use crate::gfx::{Device, Queue};

/// A recorder for graphics, compute, or transfer operations.
pub struct CmdBuffer<'a, B: Backend, C: Capability> {
    device : Rc<RefCell<Device<B>>>,
    cmd_buffer : Option<CommandBuffer<'a, B, C, MultiShot, Primary>>,
    fence : Option<B::Fence>,
}

impl<'a, B: Backend, C: Capability> Drop for CmdBuffer<'a, B, C> {
    fn drop(&mut self) {
        &self.device.borrow().get_logical_device().destroy_fence(self.fence.take().unwrap());
        debug_assert!(self.fence.is_none());
        info!("Dropped CmdBuffer")
    }
}

impl<'a, B: Backend, C: Capability> CmdBuffer<'a, B, C> {
    pub fn begin_pass() { unimplemented!() }

    pub fn end_pass() { unimplemented!() }

    pub fn get_fence(&self) -> &Option<B::Fence> {
        &self.fence
    }
}

/// Allocates the command buffers into memory for reuse.
pub struct CmdPool<B: Backend, C: Capability> {
    device : Rc<RefCell<Device<B>>>,
    cmd_pool : Option<CommandPool<B, C>>,
}

impl<B: Backend, C: Capability> Drop for CmdPool<B, C> {
    fn drop(&mut self) {
        &self.device.borrow().get_logical_device().destroy_command_pool(self.cmd_pool.take().unwrap().into_raw());
        debug_assert!(self.cmd_pool.is_none());
        info!("Dropped CmdPool")
    }
}

impl<B: Backend, C: Capability> CmdPool<B, C> {
    pub fn new(device : Rc<RefCell<Device<B>>>, queue : &mut Queue<B, C>) -> Self {
        let cmd_pool = Some(device
            .borrow()
            .get_logical_device()
            .create_command_pool_typed(queue.get_queue_group_mut(),
                                       CommandPoolCreateFlags::RESET_INDIVIDUAL, num_cpus::get())
            .expect("Failed to create command pool"));
        Self { device, cmd_pool }
    }

    pub fn get_cmd_buffer(&mut self) -> CmdBuffer<B, C> {
        let cmd_buffer = Some(self.cmd_pool.as_mut().unwrap().acquire_command_buffer(false));
        let fence = Some(self.device.borrow().get_logical_device().create_fence(true).expect("Failed to create fence."));
        CmdBuffer { device: self.device.clone(), cmd_buffer, fence }
    }

    pub fn reset(&mut self) {
        self.cmd_pool.as_mut().unwrap().reset();
    }

    pub fn get_cmd_pool(&mut self) -> &mut CommandPool<B, C> {
        self.cmd_pool.as_mut().unwrap()
    }
}