use std::{cell::RefCell, mem::size_of, rc::Rc};
use ash::vk;
use super::{Device, Material, util::find_memory_type_index};

pub enum BufferCreationError {
    AllocationFailed,
    UnsupportedMemoryType,
}

pub struct Buffer {
    device : Rc<RefCell<Device>>,
    buffer : vk::Buffer,
    buffer_memory : vk::DeviceMemory,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().ash_device().destroy_buffer(self.buffer, None);
            self.device.borrow().ash_device().free_memory(self.buffer_memory, None);
        }
        info!("Dropped Buffer")
    }
}

pub struct VertexBuffer {
    buffer : Buffer,
}

impl VertexBuffer {
    pub fn new(device : Rc<RefCell<Device>>, material : &Material) -> Result<Self,BufferCreationError> {
        let buffer_info = vk::BufferCreateInfo::builder()
            .size(material.vertex_buffer_size())
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .build();

        let (buffer, memory_requirements) = unsafe {
            let buffer = device
                .borrow()
                .ash_device()
                .create_buffer(&buffer_info, None)
                .expect("Failed to create buffer");
            let memory_requirements= device.borrow().ash_device().get_buffer_memory_requirements(buffer);
            (buffer, memory_requirements)
        };

        let memory_properties = device.borrow().memory_properties();
        let memory_index = find_memory_type_index(&memory_requirements, &memory_properties, vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE);
        match memory_index {
            Some(i) => {
                let allocate_info = vk::MemoryAllocateInfo::builder()
                    .memory_type_index(i)
                    .allocation_size(memory_requirements.size)
                    .build();
                let buffer_memory = unsafe {
                    let buffer_memory = device
                        .borrow()
                        .ash_device()
                        .allocate_memory(&allocate_info, None)
                        .expect("Failed to allocate memory");
                    device
                        .borrow()
                        .ash_device()
                        .bind_buffer_memory(buffer, buffer_memory, 0)
                        .expect("Failed to bind buffer memory");
                    device
                        .borrow()
                        .ash_device()
                        .map_memory(buffer_memory, 0, buffer_info.size, vk::MemoryMapFlags::empty())
                        .expect("Failed to map buffer memory");
                    // TODO: memcpy
                    device
                        .borrow()
                        .ash_device()
                        .unmap_memory(buffer_memory);
                    buffer_memory
                };

                Ok(Self {
                    buffer : Buffer { device, buffer, buffer_memory }
                })
            },
            None => Err(BufferCreationError::UnsupportedMemoryType),
        }
    }
}

pub struct IndexBuffer {
    buffer : Buffer,
}

pub struct StagingBuffer {
    buffer : Buffer,
}