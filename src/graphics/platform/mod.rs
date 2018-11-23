/// Platform specific function exports.
#[cfg(target_os = "windows")]
mod win32;
#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
pub use self::win32::{create_surface, get_required_instance_extensions};
#[cfg(target_os = "linux")]
pub use self::linux::{create_surface, get_required_instance_extensions};
