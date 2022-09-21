use std::os::raw::c_void;
use ash::extensions::{ext::DebugUtils, khr::Surface, khr::Win32Surface};
use ash::vk;
use ash::{Entry, Instance};
use winit::platform::windows::WindowExtWindows;
use winit::window::Window;

pub fn create_surface(entry : &Entry, instance : &Instance, window : &Window)
    -> vk::SurfaceKHR {
    let hwnd = window.hwnd();
    let win32_create_info = vk::Win32SurfaceCreateInfoKHR::builder()
        .hwnd(hwnd as *const c_void)
        .build();
    let win32_surface_loader = Win32Surface::new(entry, instance);
    unsafe {
        win32_surface_loader.create_win32_surface(&win32_create_info, None)
            .expect("Failed to create surface")
    }
}

pub fn get_required_instance_extensions() -> Vec<*const i8> {
    vec![Surface::name().as_ptr(), Win32Surface::name().as_ptr(), DebugUtils::name().as_ptr()] as Vec<*const i8>
}