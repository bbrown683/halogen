use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use ash::version::DeviceV1_0;
use ash::vk;
use nalgebra::{Vector2, Vector3, Vector4};
use super::Device;

pub struct VertexDescription {
    position : Vector3<f32>,
    color : Option<Vector4<f32>>,
    uv : Option<Vector2<f32>>,
}

pub struct Mesh {
    device : Rc<RefCell<Device>>,
    vertex_buffer : vk::Buffer,
    vertex_buffer_memory : vk::DeviceMemory,
//    index_buffer : vk::Buffer,
//    index_buffer_memory : vk::DeviceMemory,
    vertices : Vec<VertexDescription>,
    indices : Vec<u16>,
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().get_ash_device().destroy_buffer(self.vertex_buffer, None);
            self.device.borrow().get_ash_device().free_memory(self.vertex_buffer_memory, None);
//            self.device.borrow().get_ash_device().destroy_buffer(self.index_buffer, None);
//            self.device.borrow().get_ash_device().free_memory(self.index_buffer_memory);
        }
    }
}

impl Mesh {
    // TODO: use staging buffer.
    pub fn new(device : Rc<RefCell<Device>>) {
        let vertex_buffer_info = vk::BufferCreateInfo::builder()
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .size(mem::size_of::<VertexDescription>() as vk::DeviceSize)
            .build();

    }
}