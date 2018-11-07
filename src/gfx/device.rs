use hal::{Adapter, Backend, Device, Features, Limits, MemoryProperties, PhysicalDevice, QueueFamily };

/// This module features the lowest level types needed by the other modules for creating resources,
/// managing render state, etc.
pub struct GfxDevice<B: Backend> {
    adapter : Adapter<B>,
    features : Features,
    memory_properties : MemoryProperties,
    limits : Limits,
    device : B::Device,
    compute_queue_index : usize,
    graphics_queue_index : usize,
    transfer_queue_index : usize
}

impl<B: Backend> Drop for GfxDevice<B> {
    fn drop(&mut self) {
        // Wait for gpu operations to complete before destroying resources.
        &self.device.wait_idle().unwrap();
    }
}

impl<B: Backend> GfxDevice<B> {
    /// Creates a new rendering device for the specified adapter and surface.
    pub fn new(adapter : Adapter<B>) -> Self {
        let features = adapter.physical_device.features();
        let memory_properties = adapter.physical_device.memory_properties();
        let limits = adapter.physical_device.limits();

        let (graphics_queue_index, compute_queue_index, transfer_queue_index) =
            GfxDevice::<B>::get_queue_indices(&adapter.queue_families);
        println!("Graphics Queue Index: {:?}", graphics_queue_index);
        println!("Compute Queue Index: {:?}", compute_queue_index);
        println!("Transfer Queue Index: {:?}", transfer_queue_index);

        // Grab all queues available to us and create a vector with the queue family
        // associated with a priority.
        let mut queues = Vec::<(_, &[f32])>::new();
        for i in 0..adapter.queue_families.len() {
            queues.push((adapter.queue_families.get(i).unwrap(), &[1.0; 1]));
        }

        let gpu = adapter.physical_device.open(queues.as_slice())
            .expect("Failed to create logical device.");
        let device = gpu.device;
        Self { adapter, features, memory_properties, limits, device,
            graphics_queue_index, compute_queue_index, transfer_queue_index }
    }

    /// Returns the queue family id that was selected for each queue. The order is graphics, compute,
    /// and transfer.
    fn get_queue_indices(queue_families : &Vec<B::QueueFamily>) -> (usize, usize, usize) {
        let mut graphics_queue_index = usize::max_value();
        let mut compute_queue_index = usize::max_value();
        let mut transfer_queue_index = usize::max_value();

        for i in 0..queue_families.len() {
            let queue_family = queue_families.get(i).unwrap();
            if queue_family.supports_graphics() {
                graphics_queue_index = i;
            }
            // Grab compute queue if possible which is not being used for existing graphics operations.
            if queue_family.supports_compute() && graphics_queue_index != i {
                compute_queue_index = i;
            }
            // Get individual transfer queue if possible.
            if queue_family.supports_transfer() && !queue_family.supports_graphics() &&
                !queue_family.supports_compute() {
                transfer_queue_index = i;
            }
        }

        // This situation can arise when there may be a single queue.
        if compute_queue_index == usize::max_value() {
            compute_queue_index = graphics_queue_index;
        }

        if transfer_queue_index == usize::max_value() {
            compute_queue_index = compute_queue_index;
        }
        (graphics_queue_index, compute_queue_index, transfer_queue_index)
    }

    /// Returns a handle to the logical `Device`.
    pub fn get_device(&self) -> &B::Device {
        &self.device
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

    /// Returns a reference to the queue family for compute operations.
    pub fn get_compute_queue_family(&self) -> &B::QueueFamily {
        &self.adapter.queue_families.get(self.compute_queue_index).unwrap()
    }

    /// Returns a reference to the queue family for graphics operations.
    pub fn get_graphics_queue_family(&self) -> &B::QueueFamily {
        &self.adapter.queue_families.get(self.graphics_queue_index).unwrap()
    }

    /// Returns a reference to the queue family for transfer operations.
    pub fn get_transfer_queue_family(&self) -> &B::QueueFamily {
        &self.adapter.queue_families.get(self.transfer_queue_index).unwrap()
    }
}