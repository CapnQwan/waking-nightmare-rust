// /glwn/src/lib.rs

pub mod gl {
  #![allow(unsafe_op_in_unsafe_fn)]
  #![allow(clippy::all)]
  include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

  pub use Gles2 as Gl;
}
