use crate::editor::{editor_window::editor_window::EditorWindow};

use std::sync::Arc;

use eframe::{glow, UserEvent};
use egui_winit::winit;
use egui_glow;

struct GlutinWindowContext {
    window: winit::window::Window,
    gl_context: glutin::context::PossiblyCurrentContext,
    gl_display: glutin::display::Display,
    gl_surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
}

pub struct Editor {
  pub frame: EditorWindow,
  proxy: winit::event_loop::EventLoopProxy<UserEvent>,
  gl_window: Option<GlutinWindowContext>,
  gl: Option<Arc<glow::Context>>,
  egui_glow: Option<egui_glow::EguiGlow>,
  repaint_delay: std::time::Duration,
  clear_color: [f32; 3],
}

impl Editor {
  pub fn new(proxy: winit::event_loop::EventLoopProxy<UserEvent>) -> Self {
    Self {
      frame: EditorWindow::default(),
      proxy,
      gl_window: None,
      gl: None,
      egui_glow: None,
      repaint_delay: std::time::Duration::MAX,
      clear_color: [0.1, 0.1, 0.1],
    }
  }
}

impl eframe::App for Editor {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    self.frame.render_window(ctx, frame);
  }
}

impl winit::application::ApplicationHandler<UserEvent> for Editor {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }
}
