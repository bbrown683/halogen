//! High-level wrapper for [ash](https://github.com/MaikKlein/ash) around typical types.

mod cmd;
mod debug;
mod device;
mod framebuffer;
/// Provides initializers for commonly used Vulkan structures.
mod init;
mod instance;
mod pass;
/// Platform-specific helper functions.
mod platform;
/// Operations for a queue, such as submitting graphics, compute, or transfer operations for execution by the GPU.
mod queue;
/// Manages a Vulkan surface and swapchain, presenting the acquired images to the screen.
mod swapchain;
mod renderer;
/// Utilities for common functionality used in Vulkan.
mod util;

pub use self::renderer::Renderer;

use self::device::{Device, DeviceCreationError};
use self::instance::Instance;
use self::swapchain::{Swapchain, SwapchainCreationError};
