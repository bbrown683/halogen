use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, Capability, CommandPool, Device as LogicalDevice};
use hal::pool::{CommandPoolCreateFlags};
use crate::gfx::{Device, Queue};

pub struct CmdPool<B: Backend, C: Capability> {
    device : Rc<RefCell<Device<B>>>,
    cmd_pool : Option<CommandPool<B, C>>,
}


impl<B: Backend, C: Capability> Drop for CmdPool<B, C> {
    fn drop(&mut self) {
        &self.device.borrow().get_logical_device().destroy_command_pool(self.cmd_pool.take().unwrap().into_raw());
        debug_assert!(self.cmd_pool.is_none());
    }
}

impl<B: Backend, C: 'static> CmdPool<B, C> where C: Capability {
    pub fn new(device : Rc<RefCell<Device<B>>>, queue : Rc<RefCell<Queue<B, C>>>) -> Self {
        let cmd_pool = Some(device
            .borrow()
            .get_logical_device()
            .create_command_pool_typed(queue.borrow_mut().get_queue_group_mut(),
                                       CommandPoolCreateFlags::RESET_INDIVIDUAL,
                                       num_cpus::get())
            .expect("Failed to create command pool"));
        Self { device, cmd_pool }
    }

    pub fn get_cmd_pool(&mut self) -> &mut CommandPool<B, C> {
        self.cmd_pool.as_mut().unwrap()
    }
}