pub mod buffer;
pub mod cmd;
pub mod debug;
pub mod device;
pub mod framebuffer;
pub mod instance;
/// Defines the appearance of a renderable object. Currently provides basic options for a `ColoredMaterial` or a `TexturedMaterial`.
pub mod material;
pub mod pass;
pub mod pipeline;
/// Platform-specific helper functions.
pub mod platform;
/// Operations for a queue, such as submitting graphics, compute, or transfer operations for execution by the GPU.
pub mod queue;
/// Manages a Vulkan surface and swapchain, presenting the acquired images to the screen.
pub mod swapchain;
pub mod renderer;
/// Utilities for common functionality used in Vulkan.
pub mod util;

pub use self::renderer::Renderer;
use self::buffer::VertexBuffer;
use self::cmd::{CmdBuffer, CmdPool, CmdState};
use self::device::{Device, DeviceCreationError};
use self::framebuffer::{Framebuffer, FramebufferBuilder};
use self::instance::Instance;
use self::material::{ColoredMaterial, ColoredVertex, Material, TexturedMaterial, TexturedVertex, Vertex};
use self::pass::{RenderPass, RenderPassBuilder};
use self::pipeline::{Pipeline, PipelineBuilder};
use self::queue::Queue;
use self::swapchain::{Swapchain, SwapchainCreationError};
