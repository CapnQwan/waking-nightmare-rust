use std::ffi::CString;
use std::num::NonZeroU32;
use std::sync::Arc;

use glutin::config::ConfigTemplateBuilder;
use glutin::context::PossiblyCurrentContext;
use glutin::prelude::{GlDisplay, NotCurrentGlContext};
use glutin::surface::{GlSurface, Surface, WindowSurface};

use glwn::gl::Gl;
use winit::event_loop::EventLoop;
use winit::window::Window;

use glutin::config::Config;
use glutin::display::GetGlDisplay;

use glutin_winit::DisplayBuilder;
use glutin_winit::GlWindow;

use crate::gl_config::gl_config_picker;
use crate::gl_context::create_gl_context;

mod gl_config;
mod gl_context;

pub struct GlWindowContext {
  window: Window,
  config: Config,
  context: PossiblyCurrentContext,
  surface: Surface<WindowSurface>,
  // Note: Switch to Arc for mutli threaded resource loading?
  gl: Arc<Gl>,
}

impl GlWindowContext {
  pub fn new(event_loop: &EventLoop<()>) -> Self {
    let template = ConfigTemplateBuilder::new().with_alpha_size(8);
    let display_builder =
      DisplayBuilder::new().with_window_attributes(Some(Window::default_attributes()));

    let (window_opt, config) = display_builder
      .build(event_loop, template, gl_config_picker)
      .expect("Failed to build GL window");
    let window = window_opt.unwrap();

    let attrs = window.build_surface_attributes(Default::default()).unwrap();
    let surface = unsafe {
      config
        .display()
        .create_window_surface(&config, &attrs)
        .unwrap()
    };

    let context = create_gl_context(&window, &config)
      .make_current(&surface)
      .expect("Failed to make context current");

    let gl = Gl::load_with(|symbol| {
      let symbol = CString::new(symbol).unwrap();
      config.display().get_proc_address(symbol.as_c_str()).cast()
    });

    Self {
      window,
      config,
      context,
      surface,
      gl: Arc::new(gl),
    }
  }
}

impl GlWindowContext {
  pub fn get_gl_instance(&self) -> Arc<Gl> {
    Arc::clone(&self.gl)
  }

  pub fn resize(&self, width: NonZeroU32, height: NonZeroU32) {
    self.surface.resize(&self.context, width, height);
    unsafe {
      self
        .gl
        .Viewport(0, 0, width.get() as i32, height.get() as i32);
    }
  }

  pub fn request_redraw(&self) {
    self.window.request_redraw();
  }

  pub fn swap_buffers(&self) {
    let _ = self.surface.swap_buffers(&self.context);
  }

  pub fn get_window(&self) -> &Window {
    &self.window
  }

  pub fn get_context(&self) -> &Window {
    &self.window
  }

  pub fn get_surface(&self) -> &Window {
    &self.window
  }
}
