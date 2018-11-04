use hal::{format, image, pass, pool};
use hal::pso::PipelineStage;
use hal::{Adapter, Backend, CommandPool, Device, Features, Graphics, Limits, MemoryProperties,
          PhysicalDevice, QueueGroup, Surface };

pub struct GfxDevice<B: Backend> {
    pub physical_device : B::PhysicalDevice,
    pub enabled_features : Features,
    pub memory_properties : MemoryProperties,
    pub limits : Limits,
    pub logical_device : B::Device,
    pub queue_group : QueueGroup<B, Graphics>,
    pub render_pass : Option<B::RenderPass>,
    pub command_pool : Option<CommandPool<B, Graphics>>
}

impl<B: Backend> Drop for GfxDevice<B> {
    fn drop(&mut self) {
        // Wait for gpu operations to complete before destroying resources.
        &self.logical_device.wait_idle().unwrap();

        &self.logical_device.destroy_render_pass(self.render_pass.take().unwrap());
        debug_assert!(self.render_pass.is_none());
        &self.logical_device.destroy_command_pool(self.command_pool.take().unwrap().into_raw());
        debug_assert!(self.command_pool.is_none());
    }
}

impl<B: Backend> GfxDevice<B> {
    pub fn new(adapter : Adapter<B>, surface : &B::Surface) -> Self {
        let (logical_device, queue_group) = adapter
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
            .create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::empty(), num_cpus::get())
            .expect("Failed to create command pool"));

        let physical_device  = adapter.physical_device;
        let enabled_features = physical_device.features();
        let memory_properties = physical_device.memory_properties();
        let limits = physical_device.limits();

        Self { physical_device, enabled_features, memory_properties, limits, logical_device, queue_group,
            render_pass, command_pool }
    }
}