use core;
use glow::*;
use web_sys;
use wasm_bindgen;
use wasm_bindgen::JsCast;

pub fn init() {
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

  // This could be called from `requestAnimationFrame`, a winit event loop, etc.
  // gl.clear(glow::COLOR_BUFFER_BIT);
  // gl.draw_arrays(glow::TRIANGLES, 0, 3);
  // gl.delete_program(program);
  // gl.delete_vertex_array(vertex_array);
}