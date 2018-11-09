use winit::dpi::{LogicalPosition, LogicalSize};

pub trait CapturedEvent {
    fn on_resize(&mut self, size : LogicalSize);
    fn on_cursor_move(&mut self, position : LogicalPosition);
}