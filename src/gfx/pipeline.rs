use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend};
use crate::gfx::{Device, Material};

pub struct Pipeline<B: Backend> {
    device : Rc<RefCell<Device<B>>>,
    layout : B::PipelineLayout,
}

impl<B: Backend> Drop for Pipeline<B> {
    fn drop(&mut self) {

    }
}

pub struct PipelineBuilder {
    material : Material,
}
