use crate::math::Vector3;

#[derive(Clone, Copy)]
pub struct Transform {
  pub position: Vector3,
  pub rotation: Quaternion,
  pub scale: Vector3,
}

impl Transform {}
