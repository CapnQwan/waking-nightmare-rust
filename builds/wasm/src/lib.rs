/// @todo
/// So apparently just using WASM will not solve TBT issues from creating shader programs.
/// But apparently using offscreen canvases can solve this issue
///
/// Also note apparently there are several issues with offscreen canvases in safari
///
/// Potentially add support for handling shaders in webgl via offscreen canvases except for in
/// safari?

#[allow(dead_code)]
pub fn init() {
  #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
  {
    use core;
    use glow::*;
    use wasm_bindgen;
    use wasm_bindgen::JsCast;
    use web_sys;

    let canvas = web_sys::window()
      .unwrap()
      .document()
      .unwrap()
      .get_element_by_id("canvas")
      .unwrap()
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .unwrap();
    let webgl2_context = canvas
      .get_context("webgl2")
      .unwrap()
      .unwrap()
      .dyn_into::<web_sys::WebGl2RenderingContext>()
      .unwrap();
    let gl = Context::from_webgl2_context(webgl2_context);

    core::run_engine(gl, "#version 300 es".to_string());
  }
  // This could be called from `requestAnimationFrame`, a winit event loop, etc.
  // gl.clear(glow::COLOR_BUFFER_BIT);
  // gl.draw_arrays(glow::TRIANGLES, 0, 3);
  // gl.delete_program(program);
  // gl.delete_vertex_array(vertex_array);
}
