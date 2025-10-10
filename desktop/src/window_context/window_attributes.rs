use winit::window::{Window, WindowAttributes};

pub fn window_attributes() -> WindowAttributes {
  Window::default_attributes()
    .with_transparent(true)
    .with_title("Glutin triangle gradient example (press Escape to exit)")
}
