use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, Device};
use crate::gfx::GfxDevice;

pub struct GfxSync<B :Backend> {
    device : Rc<RefCell<GfxDevice<B>>>,
    pub fence : Option<B::Fence>,
    pub present_semaphores : Option<Vec<B::Semaphore>>
}

impl<B: Backend> GfxSync<B> {
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>, image_count : u32) -> Self {
        let fence = Some(device.borrow().logical_device.create_fence(true).expect("Failed to create fence."));
        let mut present_semaphores = Vec::<B::Semaphore>::new();
        let mut current_count  = 0;
        while current_count < image_count {
            present_semaphores.push(device.borrow().logical_device.create_semaphore().expect("Failed to create semaphore."));
            current_count = current_count + 1;
        }
        Self { device, fence, present_semaphores: Some(present_semaphores) }
    }
}

impl<B: Backend> Drop for GfxSync<B> {
    fn drop(&mut self) {
        &self.device.borrow().logical_device.destroy_fence(self.fence.take().unwrap());
        debug_assert!(self.fence.is_none());
        for present_semaphore in self.present_semaphores.take().unwrap() {
            &self.device.borrow().logical_device.destroy_semaphore(present_semaphore);
        }
        debug_assert!(self.present_semaphores.is_none());
    }
}