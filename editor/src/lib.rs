#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use core::gl::Gl;
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

fn create_gl_window() -> (Window, Gl) {
  let event_loop = EventLoop::new().unwrap();

  let display_builder =
    DisplayBuilder::new().with_window_attributes(Some(Window::default_attributes()));

  let (window, gl_config) =
    match display_builder
      .clone()
      .build(event_loop, self.template.clone(), gl_config_picker)
    {
      Ok((window, gl_config)) => (window.unwrap(), gl_config),
      Err(err) => {
        self.exit_state = Err(err);
        event_loop.exit();
        return;
      }
    };

  println!("Picked a config with {} samples", gl_config.num_samples());

  // Mark the display as initialized to not recreate it on resume, since the
  // display is valid until we explicitly destroy it.
  self.gl_display = GlDisplayCreationState::Init;

  // Create gl context.
  self.gl_context = Some(create_gl_context(&window, &gl_config).treat_as_possibly_current());

  (window, gl_config)
}
