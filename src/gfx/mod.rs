//! High-level wrapper for [ash](https://github.com/MaikKlein/ash) around typical types.

pub mod cmd;
pub mod debug;
pub mod device;
pub mod framebuffer;
/// Provides initializers for commonly used Vulkan structures.
pub mod init;
pub mod instance;
pub mod pass;
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
use self::device::{Device, DeviceCreationError};
use self::framebuffer::Framebuffer;
use self::instance::Instance;
use self::queue::Queue;
use self::pass::RenderPass;
use self::swapchain::{Swapchain, SwapchainCreationError};
