mod math;
mod render;

pub mod gl {
  #![allow(clippy::all)]
  include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

  pub use Gles2 as Gl;
}

//pub fn run_engine(gl: Context, shader_version: String) {}
