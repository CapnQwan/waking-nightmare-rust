use std::sync::Arc;

use egui::{Context as EguiContext, FullOutput, ViewportId};
use egui_glow::{Painter, ShaderVersion};
use egui_winit::State as EguiWinit;
use glow::HasContext;
use winit::window::Window;

/// Helper that sets up egui + glow using an existing GL window
pub struct EguiGlowSupport {
  pub ctx: EguiContext,
  pub state: EguiWinit,
  pub painter: Painter,
  pub glow_ctx: Arc<glow::Context>,
}

impl EguiGlowSupport {
  pub fn new(window: &Window, get_proc_address: impl Fn(&str) -> *const std::ffi::c_void) -> Self {
    // --- create glow context ---
    let glow_ctx = unsafe { glow::Context::from_loader_function(|s| get_proc_address(s)) };
    let glow_ctx = Arc::new(glow_ctx);

    // --- egui setup ---
    let ctx: EguiContext = EguiContext::default();
    // `egui_winit::State::new` takes an owned `Context`, so clone here to
    // avoid moving `ctx` out of this function (Context is cheaply cloneable).
    let state = EguiWinit::new(ctx.clone(), ViewportId::ROOT, window, None, None, None);

    // --- egui_glow painter ---
    // @todo - update the ShaderVersion based on the current output type (WASM / DESKTOP)
    let painter = Painter::new(glow_ctx.clone(), "", Some(ShaderVersion::Es300), false)
      .expect("Failed to create egui glow painter");

    Self {
      ctx,
      state,
      painter,
      glow_ctx,
    }
  }

  /// Begin a new egui frame.
  pub fn begin_frame(&mut self, window: &Window) {
    let raw_input = self.state.take_egui_input(window);
    self.ctx.begin_pass(raw_input);
  }

  /// Run egui UI logic.
  pub fn ui<F: FnOnce(&EguiContext)>(&mut self, f: F) {
    f(&self.ctx);
  }

  /// End frame and paint results.
  pub fn paint(&mut self, window: &Window) {
    let FullOutput {
      platform_output,
      textures_delta,
      shapes,
      ..
    } = self.ctx.end_pass();

    self.state.handle_platform_output(window, platform_output);

    let paint_jobs = self.ctx.tessellate(shapes, self.ctx.pixels_per_point());

    let size = window.inner_size();
    unsafe {
      self
        .glow_ctx
        .viewport(0, 0, size.width as i32, size.height as i32);
    }

    self.painter.paint_and_update_textures(
      [size.width, size.height],
      self.ctx.pixels_per_point(),
      &paint_jobs,
      &textures_delta,
    );
  }
}
