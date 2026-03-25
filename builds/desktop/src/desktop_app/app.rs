use core::engine::Core;
use core::traits::Plugin;
use gl_window_context::GlWindowContext;
use time::TimePlugin;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey};

use core::create_engine_instance;
use std::num::NonZeroU32;

pub struct App {
  window_context: GlWindowContext,
  engine: Core,
}

impl App {
  pub fn new(event_loop: &EventLoop<()>) -> Self {
    let window_context = GlWindowContext::new(event_loop);
    let mut engine = create_engine_instance();
    TimePlugin::register(&mut engine);

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
    self.engine.tick();
    self.window_context.request_redraw();
    self.window_context.swap_buffers();
  }
}
