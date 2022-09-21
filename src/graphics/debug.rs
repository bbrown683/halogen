use std::ffi::CStr;
use std::os::raw::c_void;
use ash::vk;

pub unsafe extern "system" fn debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT, 
    message_types: vk::DebugUtilsMessageTypeFlagsEXT, 
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT, 
    p_user_data: *mut c_void
) -> vk::Bool32 {
    let message = (*p_callback_data).p_message;
    match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => debug!("{:?}", CStr::from_ptr(message)),
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => info!("{:?}", CStr::from_ptr(message)),
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => warn!("{:?}", CStr::from_ptr(message)),
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => error!("{:?}", CStr::from_ptr(message)),
        _ => ()
    }    
    vk::FALSE
}