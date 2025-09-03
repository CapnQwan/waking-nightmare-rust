#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::{UserEvent};
use winit::event_loop::EventLoop;
use crate::editor::editor::Editor;

mod editor;

pub fn init() {
  let event_loop = EventLoop::<UserEvent>::with_user_event()
      .build()
      .unwrap();
  let proxy = event_loop.create_proxy();

  let mut app = Editor::new(proxy);

  event_loop.run_app(&mut app).expect("failed to run app");

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


