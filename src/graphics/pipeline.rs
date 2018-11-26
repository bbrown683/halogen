use std::cell::RefCell;
use std::default::Default;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::mem::size_of;
use std::rc::Rc;
use ash::version::DeviceV1_0;
use ash::vk;
use spirv_reflect::ShaderModule;
use super::{Device, RenderPass};
use super::reflect;

/// Represents the flow of the graphics pipeline from the vertex to fragment stage.
pub struct Pipeline {
    device : Rc<RefCell<Device>>,
    pipeline : vk::Pipeline,
    layout : vk::PipelineLayout,
    supports_compute : bool,
    supports_graphics : bool,
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().get_ash_device().device_wait_idle().unwrap();
            self.device.borrow().get_ash_device().destroy_pipeline_layout(self.layout, None);
            self.device.borrow().get_ash_device().destroy_pipeline(self.pipeline, None);
        }
        info!("Dropped GraphicsPipeline")
    }
}

impl Pipeline {
    pub fn get_pipeline_raw(&self) -> vk::Pipeline {
         self.pipeline
    }

    pub fn get_layout_raw(&self) -> vk::PipelineLayout {
         self.layout
    }

    pub fn supports_compute(&self) -> bool {
        self.supports_compute
    }

    pub fn supports_graphics(&self) -> bool {
        self.supports_graphics
    }
}

pub struct PipelineBuilder {
    device : Rc<RefCell<Device>>,
    shaders : Vec<(Vec<u8>, ShaderModule)>
}

impl PipelineBuilder {
    /// Creates a new pipeline using the initial shader. This would be either a compute shader, or a vertex shader.
    pub fn new(device : Rc<RefCell<Device>>) -> Self {
        Self { device,
            shaders: Vec::new()
        }
    }

    /// Adds a shader directly from bytes. Useful when combined with the `include_bytes` macro
    pub fn add_shader_from_bytes(mut self, bytes : Vec<u8>) -> Self {
        let shader_module = ShaderModule::load_u8_data(bytes.as_slice());
        match shader_module {
            Ok(module) => self.shaders.push((bytes, module)),
            Err(error) => error!("Failed to load module! Error: {:?}", error)
        }
        self
    }

    /// Adds a shader to the pipeline from a file path.
    pub fn add_shader_from_path(mut self, path : &str) -> Self {
        let mut file = File::open(path).expect("Failed to open file for reading.");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read contents as bytes.");
        let shader_module = ShaderModule::load_u8_data(contents.as_slice());
        match shader_module {
            Ok(module) => self.shaders.push((contents, module)),
            Err(error) => error!("Failed to load module! Error: {:?}", error)
        }
        self
    }

    /// Builds a graphics pipeline.
    pub fn build_graphics(self, render_pass : &RenderPass, extent : vk::Extent2D) -> Pipeline {
        let entry_point = CString::new("main").unwrap();
        let mut vertex_input = vk::PipelineVertexInputStateCreateInfo::builder();

        let mut modules = Vec::new();
        let mut stages = Vec::new();
        for shader in self.shaders {
            let stage = reflect::get_shader_stage(shader.1.get_shader_stage());
            let bindings = shader.1.enumerate_descriptor_bindings(None).unwrap();
            let sets = shader.1.enumerate_descriptor_sets(None).unwrap();

            // Check for vertex stage as we need to reflect its input variables to build the vertex input state.
            if stage == vk::ShaderStageFlags::VERTEX {
                let inputs = shader.1.enumerate_input_variables(None);
                match inputs {
                    Ok(variables) => {
                        let mut attributes = Vec::new();
                        for variable in variables {
                            let attribute_desc = attributes.push(vk::VertexInputAttributeDescription::builder()
                                .location(variable.location)
                                .offset(variable.word_offset)
                                .format(reflect::get_shader_input_format(variable.format))
                                .build());
                        }
                        // Feels weird to write this. Cleanup in future.
                        vertex_input.clone().p_vertex_attribute_descriptions = attributes.as_ptr();
                        vertex_input.clone().vertex_attribute_description_count = attributes.len() as u32;
                    },
                    Err(error) => println!("{:?}", error)
                }
            }

            // create shader module.
            let module_info = vk::ShaderModuleCreateInfo {
                p_code: shader.0.as_ptr() as *const u32,
                code_size: shader.0.len(),
                ..Default::default()
            };

            let module = unsafe {
                self.device
                    .borrow()
                    .get_ash_device()
                    .create_shader_module(&module_info, None)
                    .unwrap()
            };

            let stage_info = vk::PipelineShaderStageCreateInfo::builder()
                .module(module)
                .stage(stage)
                .name(entry_point.as_c_str())
                .build();

            modules.push(module);
            stages.push(stage_info);
        }

        let color_blend_attachments = vec![
            vk::PipelineColorBlendAttachmentState::builder()
                .color_write_mask(vk::ColorComponentFlags::R | vk::ColorComponentFlags::G |
                    vk::ColorComponentFlags::B | vk::ColorComponentFlags::A).build()];

        let color_blend_info = vk::PipelineColorBlendStateCreateInfo::builder()
            .attachments(color_blend_attachments.as_slice());

        let input_assembly_info = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST);

        let multisample_info = vk::PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let rasterizer_info = vk::PipelineRasterizationStateCreateInfo::builder()
            .line_width(1.0);

        let viewports = vec![
            vk::Viewport::builder()
                .width(extent.width as _)
                .height(extent.height as _)
                .max_depth(1.0).build()];
        let scissors = vec![
            vk::Rect2D::builder()
                .extent(extent).build()];
        let viewport_info = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(viewports.as_slice())
            .scissors(scissors.as_slice());

        // Currently we are not using this for anything, but in the future this will hold
        // information relating to bindings. A layout is required for creating a pipeline.
        let layout_info = vk::PipelineLayoutCreateInfo::builder();

        let layout = unsafe {
            self.device
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
            let pipeline = self.device
                .borrow()
                .get_ash_device()
                .create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
                .expect("Failed to create pipeline").remove(0);
            // Destroy modules which are now unneeded.
            for module in modules {
                self.device.borrow().get_ash_device().destroy_shader_module(module, None);
            }
            (pipeline)
        };
        Pipeline { device: self.device,
            pipeline,
            layout,
            supports_graphics: true,
            supports_compute: false,
        }
    }

    pub fn build_compute(self) {

    }
}