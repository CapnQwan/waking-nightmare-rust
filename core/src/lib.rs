use crate::math::{Matrix2x2, Matrix3x3, Matrix4x4, Quaternion, Transform, Vector3};

mod math;
mod rendering;

pub mod gl {
  #![allow(clippy::all)]
  include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

  pub use Gles2 as Gl;
}

//pub fn run_engine(gl: Context, shader_version: String) {}

pub fn test() {
  let _vec = Vector3::default();
  let _quaternion = Quaternion::default();
  let _transform = Transform::default();
  let _m2x2 = Matrix2x2::default();
  let _m3x3 = Matrix3x3::default();
  let _m4x4 = Matrix4x4::default();
}
