#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use core::gl::Gl;
use desktop::window_context::{create_gl_context, gl_config_picker};
use glutin::config::{Config, ConfigTemplateBuilder};
use glutin::context::PossiblyCurrentContext;
use glutin::prelude::NotCurrentGlContext;
use glutin_winit::DisplayBuilder;
use winit::window::Window;

use winit::event_loop::EventLoop;

mod editor;

pub fn init() {
  //let event_loop = EventLoop::<editor_old::editor::UserEvent>::with_user_event()
  //  .build()
  //  .unwrap();
  //let proxy = event_loop.create_proxy();

  //let mut app = Editor::new(proxy);

  //event_loop.run_app(&mut app).expect("failed to run app");

  // let options = eframe::NativeOptions {
  //   viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]),
  //   ..Default::default()
  // };
  // eframe::run_native(
  //   "Waking Nightmare Editor",
  //   options,
  //   Box::new(|_| {
  //     Ok(Box::<Editor>::default())
  //   }),
  // )
}

fn create_gl_window() -> (Window, Config) {
  let event_loop = EventLoop::new().unwrap();

  let template = ConfigTemplateBuilder::new().with_alpha_size(8);
  let display_builder =
    DisplayBuilder::new().with_window_attributes(Some(Window::default_attributes()));

  let (window_opt, gl_config) = display_builder
    .clone()
    .build(&event_loop, template.clone(), gl_config_picker)
    .expect("Error creating context");

  let window = window_opt.unwrap();

  (window, gl_config)
}

fn new_gl_context(window: &Window, gl_config: &Config) -> PossiblyCurrentContext {
  Some(create_gl_context(window, gl_config).treat_as_possibly_current()).unwrap()
}
