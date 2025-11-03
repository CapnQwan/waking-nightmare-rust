mod event;
mod event_bus;

// static EVENT_BUS: Lazy<std::sync::Mutex<EventBus::EventBus>> = Lazy::new(|| {
//   std::sync::Mutex::new(Editor {
//     ui: egui::CtxRef::default(),
//     selected_entity: None,
//     event_tx: EVENT_BUS.0.clone(),
//   })
// });

// pub fn get_event_bus() -> &mut EventBus {}
