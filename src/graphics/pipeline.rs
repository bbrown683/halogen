use std::cell::RefCell;
use std::default::Default;
use std::ffi::CString;
use std::mem::size_of;
use std::rc::Rc;
use ash::version::DeviceV1_0;
use ash::vk;
use super::{Device, RenderPass};

/// Represents the flow of the graphics/compute pipeline from the vertex to fragment stage.
// TODO: Create builder for this object due to somewhat complicated state. Additionally, support compute.
pub struct Pipeline {
    device : Rc<RefCell<Device>>,
    pipeline : vk::Pipeline,
    layout : vk::PipelineLayout,
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().get_ash_device().destroy_pipeline_layout(self.layout, None);
            self.device.borrow().get_ash_device().destroy_pipeline(self.pipeline, None);
        }
        info!("Dropped Pipeline")
    }
}

impl Pipeline {
    pub fn new(device : Rc<RefCell<Device>>, render_pass : &RenderPass, extent : vk::Extent2D) -> Self {
        let vertex_code = include_bytes!("../assets/shaders/default.vert.spv").to_vec();
        let fragment_code = include_bytes!("../assets/shaders/default.frag.spv").to_vec();
        let entry_point = CString::new("main").unwrap();

        let vertex_module_info = vk::ShaderModuleCreateInfo {
            p_code: vertex_code.as_ptr() as *const u32,
            code_size: vertex_code.len(),
            ..Default::default()
        };

        let fragment_module_info = vk::ShaderModuleCreateInfo {
            p_code: fragment_code.as_ptr() as *const u32,
            code_size: fragment_code.len(),
            ..Default::default()
        };

        let vertex_module = unsafe {
            device
                .borrow()
                .get_ash_device()
                .create_shader_module(&vertex_module_info, None)
                .unwrap()
        };

        let fragment_module = unsafe {
            device
                .borrow()
                .get_ash_device()
                .create_shader_module(&fragment_module_info, None)
                .unwrap()
        };

        let vertex_info = vk::PipelineShaderStageCreateInfo::builder()
            .module(vertex_module)
            .stage(vk::ShaderStageFlags::VERTEX)
            .name(entry_point.as_c_str());

        let fragment_info = vk::PipelineShaderStageCreateInfo::builder()
            .module(fragment_module)
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .name(entry_point.as_c_str());

        let stages = vec![vertex_info.build(), fragment_info.build()];

        let color_blend_attachments = vec![
            vk::PipelineColorBlendAttachmentState::builder()
                .color_write_mask(vk::ColorComponentFlags::R | vk::ColorComponentFlags::G |
                    vk::ColorComponentFlags::B | vk::ColorComponentFlags::A).build()];

        let color_blend_info = vk::PipelineColorBlendStateCreateInfo::builder()
            .attachments(color_blend_attachments.as_slice())
            .build();

        let input_assembly_info = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .build();

        let multisample_info = vk::PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1)
            .build();

        let rasterizer_info = vk::PipelineRasterizationStateCreateInfo::builder()
            .line_width(1.0)
            .build();

        let position_attribute = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .location(0)
            .offset(0)
            .build();

        let position_binding = vk::VertexInputBindingDescription::builder()
            .binding(0)
            .input_rate(vk::VertexInputRate::VERTEX)
            .stride(size_of::<f32>() as u32)
            .build();

        let vertex_attributes = vec![position_attribute];
        let vertex_bindings = vec![position_binding];

        let vertex_input = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_attribute_descriptions(vertex_attributes.as_slice())
            .vertex_binding_descriptions(vertex_bindings.as_slice())
            .build();

        let viewports = vec![
            vk::Viewport::builder()
                .width(extent.width as _)
                .height(extent.height as _)
                .max_depth(1.0)
                .build()];

        let scissors = vec![
            vk::Rect2D::builder()
                .extent(extent)
                .build()];

        let viewport_info = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(viewports.as_slice())
            .scissors(scissors.as_slice())
            .build();

        // Currently we are not using this for anything, but in the future this will hold
        // information relating to bindings. A layout is required for creating a pipeline.
        let layout_info = vk::PipelineLayoutCreateInfo::builder()
            .build();

        let layout = unsafe {
            device
                .borrow()
                .get_ash_device()
                .create_pipeline_layout(&layout_info, None)
                .unwrap()
        };

        // Build pipeline creation info.
        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .color_blend_state(&color_blend_info)
            .input_assembly_state(&input_assembly_info)
            .layout(layout)
            .multisample_state(&multisample_info)
            .rasterization_state(&rasterizer_info)
            .render_pass(render_pass.get_render_pass_raw())
            .stages(stages.as_slice())
            .vertex_input_state(&vertex_input)
            .viewport_state(&viewport_info)
            .build();

        // Create pipeline and destroy unneeded shader modules.
        let pipeline = unsafe {
            let pipeline = device
                .borrow()
                .get_ash_device()
                .create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
                .expect("Failed to create pipeline").remove(0);
            device.borrow().get_ash_device().destroy_shader_module(vertex_module, None);
            device.borrow().get_ash_device().destroy_shader_module(fragment_module, None);
            (pipeline)
        };
        Self { device, pipeline, layout }
    }
    
     pub fn get_pipeline_raw(&self) -> vk::Pipeline {
         self.pipeline
     }

     pub fn get_layout_raw(&self) -> vk::PipelineLayout {
         self.layout
     }
}