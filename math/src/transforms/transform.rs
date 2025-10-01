use crate::math::{Quaternion, Vector3};

#[derive(Clone, Copy)]
pub struct Transform {
  pub position: Vector3,
  pub rotation: Quaternion,
  pub scale: Vector3,
}

impl Default for Transform {
  fn default() -> Self {
    Transform {
      position: Vector3::default(),
      rotation: Quaternion::default(),
      scale: Vector3::default(),
    }
  }
}
