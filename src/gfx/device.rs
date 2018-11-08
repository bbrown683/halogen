use std::cell::RefCell;
use std::rc::Rc;
use std::any::TypeId;
use hal::{Adapter, Backend, Capability, Compute, Graphics, Transfer, Device, Features, Limits,
          MemoryProperties, PhysicalDevice, QueueFamily, QueueGroup };
use hal::queue::Submission;
use hal::queue::QueueFamilyId;
use crate::gfx::{GfxCmdBuffer};

pub struct GfxQueue<B: Backend, C: Capability> {
    queue_group : QueueGroup<B, C>,
}

impl<B: Backend, C: Capability> Drop for GfxQueue<B, C> {
    fn drop(&mut self) {
        // Ensure all queues in group are complete before continuing with drop.
        for queue in &self.queue_group.queues {
            queue.wait_idle().expect("Failed to wait on queue to become idle.");
        }
    }
}

impl<B: Backend, C: Capability> GfxQueue<B, C> {
    pub fn submit(cmd_buffer : Rc<RefCell<GfxCmdBuffer<B, C>>>) {

    }

    /// Returns a reference to the queue group for `Capability` operations.
    pub fn get_queue_group(&self) -> &QueueGroup<B, C> {
        &self.queue_group
    }
}

/// This module features the lowest level types needed by the other modules for creating resources,
/// managing render state, etc.
pub struct GfxDevice<B: Backend> {
    adapter : Adapter<B>,
    features : Features,
    memory_properties : MemoryProperties,
    limits : Limits,
    device : B::Device,
}

impl<B: Backend> Drop for GfxDevice<B> {
    fn drop(&mut self) {
        // Wait for gpu operations to complete before destroying resources.
        &self.device.wait_idle().unwrap();
    }
}

impl<B: Backend> GfxDevice<B> {
    /// Creates a new rendering device for the specified adapter and surface.
    pub fn new(adapter : Adapter<B>)
        -> (Option<Rc<RefCell<Self>>>, Option<GfxQueue<B, Compute>>,
            Option<GfxQueue<B, Graphics>>, Option<GfxQueue<B, Transfer>>) {
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
            GfxDevice::<B>::get_queue_ids(&adapter.queue_families);

        let mut gpu = adapter.physical_device.open(queues.as_slice())
            .expect("Failed to create logical device.");
        let device = gpu.device;
        let compute_group = Some(GfxQueue { queue_group: gpu.queues.take::<Compute>(compute_queue_id).unwrap() });
        let graphics_group = Some(GfxQueue { queue_group: gpu.queues.take::<Graphics>(graphics_queue_id).unwrap() });
        let transfer_group = Some(GfxQueue { queue_group: gpu.queues.take::<Transfer>(transfer_queue_id).unwrap() });

        (Some(Rc::new(RefCell::new(Self { adapter, features, memory_properties, limits, device}))),
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
            if queue_family.supports_transfer() && !queue_family.supports_graphics() &&
                !queue_family.supports_compute() {
                transfer_queue_id = QueueFamilyId(i);
            }
        }

        // This situation can arise when there may be a single queue.
        if compute_queue_id.0 == usize::max_value() {
            compute_queue_id = graphics_queue_id;
        }

        if transfer_queue_id.0 == usize::max_value() {
            transfer_queue_id = compute_queue_id;
        }

        println!("Graphics Queue Index: {:?}", graphics_queue_id.0);
        println!("Compute Queue Index: {:?}", compute_queue_id.0);
        println!("Transfer Queue Index: {:?}", transfer_queue_id.0);

        (graphics_queue_id, compute_queue_id, transfer_queue_id)
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

    /// Returns a reference to a queue family for the given id.
    pub fn get_queue_family(&self, id : QueueFamilyId) -> &B::QueueFamily {
        &self.adapter.queue_families.get(id.0).unwrap()
    }
}