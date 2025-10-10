use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use glutin::config::ConfigTemplateBuilder;

use glutin_winit::DisplayBuilder;

use crate::desktop_app::App;

mod desktop_app;
mod window_context;

pub fn init() {
  let event_loop = EventLoop::new().unwrap();

  let template = ConfigTemplateBuilder::new().with_alpha_size(8);
  let display_builder =
    DisplayBuilder::new().with_window_attributes(Some(Window::default_attributes()));

  event_loop.set_control_flow(ControlFlow::Poll);

  let mut app = App::new(template, display_builder);
  let _ = event_loop.run_app(&mut app);
}
