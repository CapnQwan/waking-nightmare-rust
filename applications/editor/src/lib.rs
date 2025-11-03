use winit::event_loop::{ControlFlow, EventLoop};

use crate::editor::editor::Editor;

mod editor;

pub fn init() {
  let event_loop = EventLoop::new().unwrap();
  event_loop.set_control_flow(ControlFlow::Poll);

  let mut app = Editor::new(&event_loop);
  let _ = event_loop.run_app(&mut app);
}
