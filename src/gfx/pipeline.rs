use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend};
use hal::pso::{self};
use crate::gfx::GfxDevice;

pub struct GfxPipeline<B: Backend> {
    device : Rc<RefCell<GfxDevice<B>>>,
    layout : B::PipelineLayout,
}

impl<B: Backend> Drop for GfxPipeline<B> {
    fn drop(&mut self) {

    }
}

pub struct PipelineBuilder {

}
