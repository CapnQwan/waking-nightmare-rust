use crate::{Matrix4x4, Quaternion, Vector3};

#[derive(Clone, Copy)]
pub struct Transform {
  position: Vector3,
  rotation: Quaternion,
  scale: Vector3,
  world_matrix: Matrix4x4,
  is_dirty: bool,
}

impl Transform {
  pub fn position(&self) -> Vector3 {
    self.position
  }

  fn set_position(&mut self, position: Vector3) -> &mut Self {
    self.position = position;
    self.is_dirty = true;
    self
  }

  pub fn set_x(&mut self, x: f32) -> &mut Self {
    self.position.x = x;
    self.is_dirty = true;
    self
  }

  pub fn set_y(&mut self, y: f32) -> &mut Self {
    self.position.y = y;
    self.is_dirty = true;
    self
  }

  pub fn set_z(&mut self, z: f32) -> &mut Self {
    self.position.z = z;
    self.is_dirty = true;
    self
  }

  pub fn update_world_martix(&mut self) -> &mut Self {
    self
  }
}

impl Default for Transform {
  fn default() -> Self {
    Transform {
      position: Vector3::default(),
      rotation: Quaternion::default(),
      scale: Vector3::default(),
      world_matrix: Matrix4x4::default(),
      is_dirty: true,
    }
  }
}
