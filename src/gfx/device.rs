use std::cell::RefCell;
use std::rc::Rc;
use hal::{format, image, pass, pool};
use hal::pso::PipelineStage;
use hal::{Adapter, Backend, CommandPool, Compute, Device, Features, Graphics, Transfer, Limits,
          MemoryProperties, PhysicalDevice, QueueGroup, Surface };

/// This module features the lowest level types needed by the other modules for creating resources,
/// managing render state, etc.
pub struct GfxDevice<B: Backend> {
    physical_device : Rc<RefCell<B::PhysicalDevice>>,
    features : Features,
    memory_properties : MemoryProperties,
    limits : Limits,
    logical_device : Rc<RefCell<B::Device>>,
    graphics_queues : Rc<RefCell<QueueGroup<B, Graphics>>>,
    render_pass : Option<B::RenderPass>,
    command_pool : Option<CommandPool<B, Graphics>>
}

impl<B: Backend> Drop for GfxDevice<B> {
    fn drop(&mut self) {
        // Wait for gpu operations to complete before destroying resources.
        &self.logical_device.borrow().wait_idle().unwrap();
        &self.logical_device.borrow().destroy_render_pass(self.render_pass.take().unwrap());
        &self.logical_device.borrow().destroy_command_pool(self.command_pool.take().unwrap().into_raw());
        debug_assert!(self.command_pool.is_none());
    }
}

impl<B: Backend> GfxDevice<B> {
    /// Creates a new rendering device for the specified adapter and surface.
    pub fn new(adapter : Adapter<B>, surface : &B::Surface) -> Self {
        let (logical_device, graphics_queues) = adapter
            .open_with::<_, hal::Graphics>(1, |family | surface.supports_queue_family(family))
            .expect("Failed to create device and queue group.");

        let render_pass = {
            let attachment = pass::Attachment {
                format: Some(format::Format::Rgba8Unorm),
                samples: 1,
                ops: pass::AttachmentOps::new(
                    pass::AttachmentLoadOp::Clear,
                    pass::AttachmentStoreOp::Store,
                ),
                stencil_ops: pass::AttachmentOps::DONT_CARE,
                layouts: image::Layout::Undefined..image::Layout::Present,
            };

            let subpass = pass::SubpassDesc {
                colors: &[(0, image::Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };

            let dependency = pass::SubpassDependency {
                passes: pass::SubpassRef::External..pass::SubpassRef::Pass(0),
                stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
                accesses: image::Access::empty()
                    ..(image::Access::COLOR_ATTACHMENT_READ | image::Access::COLOR_ATTACHMENT_WRITE),
            };

            Some(logical_device
                .create_render_pass(&[attachment], &[subpass], &[dependency])
                .expect("Can't create render pass"))
        };

        let command_pool = Some(logical_device
            .create_command_pool_typed(&graphics_queues, pool::CommandPoolCreateFlags::empty(), num_cpus::get())
            .expect("Failed to create command pool"));

        let physical_device  = adapter.physical_device;
        let features = physical_device.features();
        let memory_properties = physical_device.memory_properties();
        let limits = physical_device.limits();

        Self { physical_device: Rc::new(RefCell::new(physical_device)), features, memory_properties, limits,
            logical_device: Rc::new(RefCell::new(logical_device)),
            graphics_queues: Rc::new(RefCell::new(graphics_queues)),
            render_pass, command_pool }
    }

    pub fn get_physical_device(&self) -> Rc<RefCell<B::PhysicalDevice>> {
        Rc::clone(&self.physical_device)
    }

    pub fn get_enabled_features(&self) -> Features {
        self.features.clone()
    }

    pub fn get_memory_properties(&self) -> MemoryProperties {
        self.memory_properties.clone()
    }

    pub fn get_limits(&self) -> Limits {
        self.limits.clone()
    }

    pub fn get_logical_device(&self) -> Rc<RefCell<B::Device>> {
        Rc::clone(&self.logical_device)
    }

    pub fn get_graphics_queues(&self) -> Rc<RefCell<QueueGroup<B, Graphics>>> {
        Rc::clone(&self.graphics_queues)
    }

    pub fn get_render_pass(&self) -> Rc<RefCell<&Option<B::RenderPass>>> {
        Rc::from(RefCell::from(&self.render_pass))
    }

    pub fn get_command_pool(&self) -> Rc<RefCell<&Option<CommandPool<B, Graphics>>>> {
        Rc::from(RefCell::from(&self.command_pool))
    }
}