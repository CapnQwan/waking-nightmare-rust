use winit::event_loop::{ControlFlow, EventLoop};

use crate::desktop_app::App;

pub mod desktop_app;

pub fn init() {
  let event_loop = EventLoop::new().unwrap();
  event_loop.set_control_flow(ControlFlow::Poll);

  let mut app = App::new(&event_loop);
  let _ = event_loop.run_app(&mut app);
}
