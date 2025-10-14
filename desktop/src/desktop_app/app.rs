use gl_window_context::GlWindowContext;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey};

use core::Core;
use std::num::NonZeroU32;

pub struct App {
  window_context: GlWindowContext,
  engine: Core,
}

impl App {
  pub fn new(event_loop: &EventLoop<()>) -> Self {
    let window_context = GlWindowContext::new(event_loop);
    let engine = Core::new(window_context.get_gl_instance());

    Self {
      window_context,
      engine,
    }
  }
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {}

  fn suspended(&mut self, _event_loop: &ActiveEventLoop) {}

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: WindowEvent,
  ) {
    match event {
      WindowEvent::Resized(size) if size.width != 0 && size.height != 0 => {
        self.window_context.resize(
          NonZeroU32::new(size.width).unwrap(),
          NonZeroU32::new(size.height).unwrap(),
        );
      }
      WindowEvent::CloseRequested
      | WindowEvent::KeyboardInput {
        event:
          KeyEvent {
            logical_key: Key::Named(NamedKey::Escape),
            ..
          },
        ..
      } => event_loop.exit(),
      _ => (),
    }
  }

  fn exiting(&mut self, _event_loop: &ActiveEventLoop) {}

  fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
    self.engine.update();
    self.engine.draw();
    self.window_context.request_redraw();
    self.window_context.swap_buffers();
  }
}
