/// Platform specific function exports.
#[cfg(windows)]
mod win32;
#[cfg(windows)]
pub use self::win32::{create_surface, get_required_instance_extensions};