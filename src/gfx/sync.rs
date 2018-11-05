use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, Device};
use crate::gfx::GfxDevice;

/// Manages syncronization primitives, which is accessed by the `GfxSwapchain` in order to validate
/// if a image can be acquired, or waiting to submit to a queue, etc.
pub struct GfxSync<B :Backend> {
    device : Rc<RefCell<GfxDevice<B>>>,
    pub fences : Option<Vec<B::Fence>>,
    pub acquire_semaphores : Option<Vec<B::Semaphore>>,
    pub present_semaphores : Option<Vec<B::Semaphore>>
}

impl<B: Backend> GfxSync<B> {
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>, image_count : u32) -> Self {
        let mut fences = Vec::<B::Fence>::new();
        let mut acquire_semaphores = Vec::<B::Semaphore>::new();
        let mut present_semaphores = Vec::<B::Semaphore>::new();
        let mut current_count  = 0;
        while current_count < image_count {
            fences.push(device.borrow().get_logical_device().borrow().create_fence(true).expect("Failed to create fence."));
            acquire_semaphores.push(device.borrow().get_logical_device().borrow().create_semaphore().expect("Failed to create semaphore."));
            present_semaphores.push(device.borrow().get_logical_device().borrow().create_semaphore().expect("Failed to create semaphore."));
            current_count = current_count + 1;
        }
        Self { device, fences : Some(fences), acquire_semaphores: Some(acquire_semaphores), present_semaphores: Some(present_semaphores) }
    }
}

impl<B: Backend> Drop for GfxSync<B> {
    fn drop(&mut self) {
        for fence in self.fences.take().unwrap() {
            &self.device.borrow().get_logical_device().borrow().destroy_fence(fence);
        }
        debug_assert!(self.fences.is_none());
        for acquire_semaphore in self.acquire_semaphores.take().unwrap() {
            &self.device.borrow().get_logical_device().borrow().destroy_semaphore(acquire_semaphore);
        }
        debug_assert!(self.acquire_semaphores.is_none());
        for present_semaphore in self.present_semaphores.take().unwrap() {
            &self.device.borrow().get_logical_device().borrow().destroy_semaphore(present_semaphore);
        }
        debug_assert!(self.present_semaphores.is_none());
    }
}