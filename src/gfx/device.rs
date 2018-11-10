use std::cell::RefCell;
use std::rc::Rc;
use hal::{Adapter, Backend, Compute, Graphics, Transfer, Device as LogicalDevice, Features, Limits,
          MemoryProperties, PhysicalDevice, QueueFamily };
use hal::queue::QueueFamilyId;
use crate::gfx::{Queue};

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
        println!("Dropped Device")
    }
}

impl<B: Backend> Device<B> {
    /// Creates a new rendering device for the specified adapter and surface.
    pub fn new(adapter : Adapter<B>)
        -> (Option<Rc<RefCell<Self>>>, Option<Rc<RefCell<Queue<B, Compute>>>>,
            Option<Rc<RefCell<Queue<B, Graphics>>>>, Option<Rc<RefCell<Queue<B, Transfer>>>>) {
        let features = adapter.physical_device.features();
        let memory_properties = adapter.physical_device.memory_properties();
        let limits = adapter.physical_device.limits();

        // Grab all queues available to us and create a vector with the queue family
        // associated with a priority.
        let mut queues = Vec::<(_, &[f32])>::new();
        for i in 0..adapter.queue_families.len() {
            queues.push((adapter.queue_families.get(i).unwrap(), &[1.0; 1]));
        }

        let (graphics_queue_id, compute_queue_id, transfer_queue_id) =
            Device::<B>::get_queue_ids(&adapter.queue_families);

        let mut gpu = adapter.physical_device.open(queues.as_slice())
            .expect("Failed to create logical device.");
        let device = gpu.device;
        let compute_group = Some(Rc::new(RefCell::new(Queue::new(gpu.queues.take::<Compute>(compute_queue_id).unwrap()))));
        let graphics_group = Some(Rc::new(RefCell::new(Queue::new(gpu.queues.take::<Graphics>(graphics_queue_id).unwrap()))));
        let transfer_group = Some(Rc::new(RefCell::new(Queue::new(gpu.queues.take::<Transfer>(transfer_queue_id).unwrap()))));

        (Some(Rc::new(RefCell::new(Self { adapter, features, memory_properties, limits, logical_device: device }))),
         compute_group, graphics_group, transfer_group)
    }

    /// Returns the queue family id that was selected for each queue. The order is graphics, compute,
    /// and transfer.
    fn get_queue_ids(queue_families : &Vec<B::QueueFamily>)
        -> (QueueFamilyId, QueueFamilyId, QueueFamilyId) {
        let mut graphics_queue_id = QueueFamilyId(usize::max_value());
        let mut compute_queue_id = QueueFamilyId(usize::max_value());
        let mut transfer_queue_id = QueueFamilyId(usize::max_value());

        for i in 0..queue_families.len() {
            let queue_family = queue_families.get(i).unwrap();
            if queue_family.supports_graphics() {
                graphics_queue_id = QueueFamilyId(i);
            }
            // Grab compute queue if possible which is not being used for existing graphics operations.
            if queue_family.supports_compute() && graphics_queue_id.0 != i {
                compute_queue_id = QueueFamilyId(i);
            }
            // Get individual transfer queue if possible.
//            if queue_family.supports_transfer() && !queue_family.supports_graphics() &&
//                !queue_family.supports_compute() {
//                transfer_queue_id = QueueFamilyId(i);
//            }
        }

        // This situation can arise when there may be a single queue.
        if compute_queue_id.0 == usize::max_value() {
            compute_queue_id = graphics_queue_id;
        }

        if transfer_queue_id.0 == usize::max_value() {
            transfer_queue_id = compute_queue_id;
        }
        transfer_queue_id = QueueFamilyId(2);
        (graphics_queue_id, compute_queue_id, transfer_queue_id)
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
    pub fn get_features(&self) -> Features {
        self.features.clone()
    }

    /// Returns the memory properties for the current `PhysicalDevice`.
    pub fn get_memory_properties(&self) -> MemoryProperties {
        self.memory_properties.clone()
    }

    /// Returns the limits for the current `PhysicalDevice`.
    pub fn get_limits(&self) -> Limits {
        self.limits.clone()
    }

    /// Returns a reference to a queue family for the given id.
    pub fn get_queue_family(&self, id : QueueFamilyId) -> &B::QueueFamily {
        &self.adapter.queue_families.get(id.0).unwrap()
    }
}