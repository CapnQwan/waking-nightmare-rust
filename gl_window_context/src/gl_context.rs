use raw_window_handle::HasWindowHandle;
use winit::window::Window;

use glutin::config::Config;
use glutin::context::{ContextApi, ContextAttributesBuilder, NotCurrentContext, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;

pub fn create_gl_context(window: &Window, gl_config: &Config) -> NotCurrentContext {
  let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());

  // The context creation part.
  let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

  // Since glutin by default tries to create OpenGL core context, which may not be
  // present we should try gles.
  let fallback_context_attributes = ContextAttributesBuilder::new()
    .with_context_api(ContextApi::Gles(None))
    .build(raw_window_handle);

  // There are also some old devices that support neither modern OpenGL nor GLES.
  // To support these we can try and create a 2.1 context.
  let legacy_context_attributes = ContextAttributesBuilder::new()
    .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
    .build(raw_window_handle);

  // Reuse the uncurrented context from a suspended() call if it exists, otherwise
  // this is the first time resumed() is called, where the context still
  // has to be created.
  let gl_display = gl_config.display();

  unsafe {
    gl_display
      .create_context(gl_config, &context_attributes)
      .unwrap_or_else(|_| {
        gl_display
          .create_context(gl_config, &fallback_context_attributes)
          .unwrap_or_else(|_| {
            gl_display
              .create_context(gl_config, &legacy_context_attributes)
              .expect("failed to create context")
          })
      })
  }
}
