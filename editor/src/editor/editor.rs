use crate::egui_glow_support::EguiGlowSupport;
use gl_window_context::GlWindowContext;
use std::num::NonZeroU32;
use winit::{
  application::ApplicationHandler,
  event::{KeyEvent, WindowEvent},
  event_loop::{ActiveEventLoop, EventLoop},
  keyboard::{Key, NamedKey},
};

pub struct Editor {
  window_context: GlWindowContext,
  egui: EguiGlowSupport,
}

impl Editor {
  pub fn new(event_loop: &EventLoop<()>) -> Self {
    let window_context = GlWindowContext::new(event_loop);
    let window = window_context.window();

    let egui = EguiGlowSupport::new(window, |s| {
      window_context.display().get_proc_address(s) as *const _
    });

    Self {
      window_context,
      egui,
    }
  }

  fn draw(&mut self) {
    let window = self.window_context.window();
    self.egui.begin_frame(window);

    self.egui.ui(|ctx| {
      egui::Window::new("Editor UI").show(ctx, |ui| {
        ui.label("Egui + Glow + Winit Integration!");
        if ui.button("Click me").clicked() {
          println!("Clicked!");
        }
      });
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
    let window = self.window_context.window();
    self.egui.state.on_event(&self.egui.ctx, &event);

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
}
