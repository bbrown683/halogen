#![feature(extern_crate_item_prelude)]

extern crate env_logger;
extern crate gfx_hal as hal;
extern crate gfx_backend_vulkan as back;
extern crate log;
extern crate nalgebra;
extern crate num_cpus;
extern crate winit;

mod example_base;

use nalgebra::{Matrix4, Vector3};
use self::example_base::{App, Example};

pub struct Vertex {
    position : Vector3<f32>,
    color : Vector3<f32>
}

pub struct VertexUniformBlock {
    model : Matrix4<f32>,
    view : Matrix4<f32>,
    projection : Matrix4<f32>
}

pub struct TriangleExample {
    vertex : Vertex,
    ubo : VertexUniformBlock,
}

impl Example for TriangleExample {
    fn render() {

    }

    fn build_command_buffers() {

    }

    fn setup_depth_stencil() {

    }

    fn setup_frame_buffer() {

    }

    fn setup_render_pass() {

    }

    fn get_enabled_features() {

    }
}

fn main() {
    let mut app = App::new().expect("Failed to initialize application");
    app.render_loop();
}
