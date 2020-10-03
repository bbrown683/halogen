use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event::{KeyboardInput, MouseButton};

pub trait CapturedEvent {
    fn on_resize(&mut self, size : LogicalSize<f32>) {
        println!("Window was resized to {:?}", &size);
    }
    fn on_cursor_move(&mut self, position : LogicalPosition<f32>) {
        println!("Cursor was moved to {:?}", &position);
    }
    fn on_keyboard_input(&mut self, input : KeyboardInput) {
        println!("{:?} was received", input);
    }
    fn on_mouse_input(&mut self, input : MouseButton) {
        println!("{:?} was received", input);
    }
}