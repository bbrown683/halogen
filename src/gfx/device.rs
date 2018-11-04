use std::cell::RefCell;
use std::rc::Rc;
use hal::{Adapter, Backend, CommandPool, Device, Features, Graphics, Limits, MemoryProperties,
          PhysicalDevice, QueueGroup, Surface };
use hal::pool;

pub struct GfxDevice<B: Backend> {
    pub physical_device : B::PhysicalDevice,
    enabled_features : Features,
    memory_properties : MemoryProperties,
    limits : Limits,
    pub logical_device : B::Device,
    pub queue_group : QueueGroup<B, Graphics>,
    pub command_pool : Option<CommandPool<B, Graphics>>
}

impl<B: Backend> Drop for GfxDevice<B> {
    fn drop(&mut self) {
        // Wait for gpu operations to complete before destroying resources.
        &self.logical_device.wait_idle().unwrap();
        &self.logical_device.destroy_command_pool(self.command_pool.take().unwrap().into_raw());
        debug_assert!(self.command_pool.is_none());
    }
}

impl<B: Backend> GfxDevice<B> {
    pub fn new(adapter : Adapter<B>, surface : &B::Surface) -> Self {
        let (logical_device, queue_group) = adapter
            .open_with::<_, hal::Graphics>(1, |family | surface.supports_queue_family(family))
            .expect("Failed to create device and queue group.");
        let command_pool = Some(logical_device
            .create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::empty(), num_cpus::get())
            .expect("Failed to create command pool"));

        let physical_device  = adapter.physical_device;
        let enabled_features = physical_device.features();
        let memory_properties = physical_device.memory_properties();
        let limits = physical_device.limits();
        Self { physical_device, enabled_features, memory_properties, limits, logical_device, queue_group,
            command_pool }
    }

    pub fn get_enabled_features(self) -> Features {
        self.enabled_features.clone()
    }

    pub fn get_limits(self) -> Limits {
        self.limits.clone()
    }

    pub fn get_memory_properties(self) -> MemoryProperties {
        self.memory_properties.clone()
    }
}