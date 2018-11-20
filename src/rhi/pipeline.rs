use std::cell::RefCell;
use std::rc::Rc;
use ash::vk;
use super::{Device, RenderPass};

pub struct Pipeline {
    device : Rc<RefCell<Device>>,
    // pipeline : vk::Pipeline,
    // layout : vk::PipelineLayout,
}

impl Drop for Pipeline {
    fn drop(&mut self) {

    }
}

impl Pipeline {
    pub fn new(device : Rc<RefCell<Device>>, render_pass : &RenderPass) -> Self {
        let vertex_info = vk::PipelineShaderStageCreateInfo::builder()
            .build();

        let fragment_info = vk::PipelineShaderStageCreateInfo::builder()
            .build();

        let stages = vec![vertex_info, fragment_info];

        let viewport_info = vk::PipelineViewportStateCreateInfo::builder()
            .build();

        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(stages.as_slice())
            .viewport_state(&viewport_info)
            .build();
        Self { device }
    }
    
    // pub fn get_pipeline_raw(&self) -> vk::Pipeline {
    //     self.pipeline
    // }

    // pub fn get_layout_raw(&self) -> vk::PipelineLayout {
    //     self.layout
    // }
}