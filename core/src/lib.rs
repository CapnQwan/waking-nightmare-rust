pub use crate::engine::EngineContext;

// @todo - delete later opting for game (project) based assets to keep core clean
mod assets;
mod engine;
mod math;

pub mod gl {
  #![allow(clippy::all)]
  include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

  pub use Gles2 as Gl;
}

pub fn create_engine_instance(gl: gl::Gles2) -> EngineContext {
  EngineContext::new(gl)
}
