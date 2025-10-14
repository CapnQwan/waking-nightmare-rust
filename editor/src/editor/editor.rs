use core::{Core, create_engine_instance};
use gl_window_context::GlWindowContext;
use glutin::prelude::GlDisplay;
use std::ffi::CString;
use std::num::NonZeroU32;
use winit::{
  application::ApplicationHandler,
  event::{KeyEvent, WindowEvent},
  event_loop::{ActiveEventLoop, EventLoop},
  keyboard::{Key, NamedKey},
};

use crate::editor::{editor_window::EditorWindow, egui_glow_support::EguiGlowSupport};

pub struct Editor {
  window_context: GlWindowContext,
  engine: Core,
  egui: EguiGlowSupport,
  editor_window: EditorWindow,
}

impl Editor {
  pub fn new(event_loop: &EventLoop<()>) -> Self {
    let window_context = GlWindowContext::new(event_loop);
    let window = window_context.get_window();

    let engine = create_engine_instance(window_context.get_gl_instance());

    let egui = EguiGlowSupport::new(window, |symbol| {
      let symbol = CString::new(symbol).unwrap();
      window_context.display().get_proc_address(symbol.as_c_str()) as *const _
    });

    let editor_window = EditorWindow::default();

    Self {
      window_context,
      engine,
      egui,
      editor_window,
    }
  }

  fn draw(&mut self) {
    // @todo - Only update when the game is running
    self.engine.update();

    let window = self.window_context.get_window();
    self.egui.begin_frame(window);

    self.engine.draw();

    self.egui.ui(|ctx| {
      self.editor_window.render_window(ctx);
    });

    self.egui.paint(window);

    self.window_context.swap_buffers();
  }
}

impl ApplicationHandler for Editor {
  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _id: winit::window::WindowId,
    event: WindowEvent,
  ) {
    let window = self.window_context.get_window();
    // egui_winit::State expects (window, event) — the `State` already holds the Context
    let _ = self.egui.state.on_window_event(window, &event);

    match event {
      WindowEvent::CloseRequested
      | WindowEvent::KeyboardInput {
        event:
          KeyEvent {
            logical_key: Key::Named(NamedKey::Escape),
            ..
          },
        ..
      } => event_loop.exit(),

      WindowEvent::Resized(size) if size.width > 0 && size.height > 0 => {
        self.window_context.resize(
          NonZeroU32::new(size.width).unwrap(),
          NonZeroU32::new(size.height).unwrap(),
        );
      }

      WindowEvent::RedrawRequested => {
        self.draw();
      }

      _ => {}
    }
  }

  fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
    self.window_context.request_redraw();
  }

  fn resumed(&mut self, event_loop: &ActiveEventLoop) {}
}
