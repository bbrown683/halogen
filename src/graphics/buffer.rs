use std::{cell::RefCell, mem::size_of, rc::Rc};
use ash::version::DeviceV1_0;
use ash::vk;
use super::{Device, Vertex};

pub struct VertexBuffer {
    device : Rc<RefCell<Device>>,

}

impl VertexBuffer {
    pub fn new<V: Vertex>(device : Rc<RefCell<Device>>) -> Self {
        let size = size_of::<V>();
        Self { device }
    }
}