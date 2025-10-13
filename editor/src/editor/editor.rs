use core::Core;
use std::num::NonZeroU32;

use egui::Context as EguiContext;
use egui_glow::Painter as EGuiPainter;
use egui_winit::State as EguiWinit;
use gl_window_context::GlWindowContext;
use winit::{
  application::ApplicationHandler,
  event::{KeyEvent, WindowEvent},
  event_loop::{ActiveEventLoop, EventLoop},
  keyboard::{Key, NamedKey},
};

pub struct Editor {
  window_context: GlWindowContext,
  engine: Core,
  egui_painter: Painter,
}

impl Editor {
  pub fn new(event_loop: &EventLoop<()>) -> Self {
    let window_context = GlWindowContext::new(event_loop);
    let gl = window_context.get_gl_instance();
    let engine = Core::new(gl);

    // Create egui context + platform + painter
    let egui_ctx = EguiContext::default();
    let egui_winit = EguiWinit::new(&event_loop);
    let egui_painter = EGuiPainter::new(gl, layer_id, clip_rect);

    Self {
      window_context,
      engine,
      egui_glow,
    }
  }
}

impl ApplicationHandler for Editor {
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
    self.window_context.request_redraw();
    self.window_context.swap_buffers();
  }
}
