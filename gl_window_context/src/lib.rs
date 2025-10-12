use glutin::config::ConfigTemplateBuilder;
use glutin::context::PossiblyCurrentContext;
use glutin::prelude::{GlDisplay, NotCurrentGlContext};
use glutin::surface::{Surface, WindowSurface};

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
mod window_attributes;

pub struct GlWindowContext {
  window: Window,
  config: Config,
  context: PossiblyCurrentContext,
  surface: Surface<WindowSurface>,
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

    let context = create_gl_context(&window, &config).treat_as_possibly_current();

    let attrs = window.build_surface_attributes(Default::default()).unwrap();
    let surface = unsafe {
      config
        .display()
        .create_window_surface(&config, &attrs)
        .unwrap()
    };

    Self {
      window,
      config,
      context,
      surface,
    }
  }
}
