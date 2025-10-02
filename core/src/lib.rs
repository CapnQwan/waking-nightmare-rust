pub use crate::engine::Core;

// @todo - delete later opting for game (project) based assets to keep core clean
mod assets;
mod engine;
mod traits;

pub mod gl {
  #![allow(unsafe_op_in_unsafe_fn)]
  #![allow(clippy::all)]
  include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

  pub use Gles2 as Gl;
}

pub fn create_engine_instance(gl: gl::Gles2) -> Core {
  Core::new(gl)
}
