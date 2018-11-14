use std::cell::RefCell;
use std::rc::Rc;
use hal::{Adapter, Backend, Compute, Graphics, Transfer, Device as LogicalDevice, Features, General,
          Limits, MemoryProperties, PhysicalDevice, QueueFamily, Surface };
use hal::queue::{QueueFamilyId, QueueType};
use crate::gfx::Queue;

/// This module features the lowest level types needed by the other modules for creating resources,
/// managing render state, etc.
pub struct Device<B: Backend> {
    adapter : Adapter<B>,
    features : Features,
    memory_properties : MemoryProperties,
    limits : Limits,
    logical_device: B::Device,
}

impl<B: Backend> Drop for Device<B> {
    fn drop(&mut self) {
        // Wait for gpu operations to complete before destroying resources.
        &self.logical_device.wait_idle().unwrap();
        info!("Dropped Device")
    }
}

impl<B: Backend> Device<B> {
    /// Creates a new rendering device for the specified adapter and surface.
    pub fn new(adapter : Adapter<B>, surface : &B::Surface)
        -> (Option<Rc<RefCell<Self>>>,
            Option<Rc<RefCell<Queue<B, Compute>>>>,
            Option<Rc<RefCell<Queue<B, Graphics>>>>,
            Option<Rc<RefCell<Queue<B, Transfer>>>>,
            Option<Rc<RefCell<Queue<B, Graphics>>>>) {
        let features = adapter.physical_device.features();
        let memory_properties = adapter.physical_device.memory_properties();
        let limits = adapter.physical_device.limits();

        // Grab all queues available to us and create a vector with the queue family
        // associated with a priority.
        // Additionally find each queue family of the specified type, individually,
        // if possible.
        let mut present_queue_id = QueueFamilyId(0);
        let mut graphics_queue_id = QueueFamilyId(0);
        let mut compute_queue_id = QueueFamilyId(0);
        let mut transfer_queue_id = QueueFamilyId(0);

        let mut queues = Vec::<(_, &[f32])>::new();
        for i in 0..adapter.queue_families.len() {
            let queue_family = adapter.queue_families.get(i).unwrap();
            queues.push((queue_family, &[1.0; 1]));
            if surface.supports_queue_family(queue_family) {
                present_queue_id = QueueFamilyId(i);
            }
            if queue_family.queue_type() == QueueType::General ||
                queue_family.queue_type() == QueueType::Graphics {
                graphics_queue_id = QueueFamilyId(i);
            }
            if queue_family.queue_type() == QueueType::Compute {
                compute_queue_id = QueueFamilyId(i);
            }
            if queue_family.queue_type() == QueueType::Transfer {
                transfer_queue_id = QueueFamilyId(i);
            }
        }

        let mut gpu = adapter.physical_device.open(queues.as_slice())
            .expect("Failed to create logical device.");
        let device = gpu.device;
        let compute_group = Some(Rc::new(RefCell::new(Queue::new(gpu.queues.take::<Compute>(compute_queue_id).unwrap()))));
        let graphics_group = Some(Rc::new(RefCell::new(Queue::new(gpu.queues.take::<Graphics>(graphics_queue_id).unwrap()))));
        let transfer_group = Some(Rc::new(RefCell::new(Queue::new(gpu.queues.take::<Transfer>(transfer_queue_id).unwrap()))));
        let present_group = Some(Rc::clone(&graphics_group.clone().unwrap()));

        (Some(Rc::new(RefCell::new(Self { adapter, features, memory_properties, limits, logical_device: device }))),
         compute_group, graphics_group, transfer_group, present_group)
    }

    /// Returns a handle to the logical `Device`.
    pub fn get_logical_device(&self) -> &B::Device {
        &self.logical_device
    }

    /// Returns a handle to the logical `Device`.
    pub fn get_logical_device_mut(&mut self) -> &B::Device {
        &self.logical_device
    }

    /// Returns a handle to the `PhysicalDevice`.
    pub fn get_physical_device(&self) -> &B::PhysicalDevice {
        &self.adapter.physical_device
    }

    /// Returns the features for the current `PhysicalDevice`.
    pub fn get_features(self) -> Features {
        self.features.clone()
    }

    /// Returns the memory properties for the current `PhysicalDevice`.
    pub fn get_memory_properties(self) -> MemoryProperties {
        self.memory_properties.clone()
    }

    /// Returns the limits for the current `PhysicalDevice`.
    pub fn get_limits(self) -> Limits {
        self.limits.clone()
    }

    /// Returns a reference to a queue family for the given id.
    pub fn get_queue_family(&self, id : QueueFamilyId) -> &B::QueueFamily {
        &self.adapter.queue_families.get(id.0).unwrap()
    }
}