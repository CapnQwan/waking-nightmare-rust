use crate::{FloatVector, Matrix4x4, Quaternion, Vector3};

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

  pub fn rotation(&self) -> Quaternion {
    self.rotation
  }

  pub fn scale(&self) -> Vector3 {
    self.scale
  }

  pub fn world_matrix(&self) -> Matrix4x4 {
    self.world_matrix
  }

  pub fn set_position(&mut self, position: Vector3) -> &mut Self {
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

  pub fn set_rotation(&mut self, rotation: Quaternion) -> &mut Self {
    self.rotation = rotation;
    self.is_dirty = true;
    self
  }

  pub fn update_world_matrix(&mut self) -> &mut Self {
    if self.is_dirty {
      self.world_matrix = Matrix4x4::transform_matrix(*self);
      self.is_dirty = false;
    }
    self
  }

  pub fn forward(&self) -> Vector3 {
    let m = Matrix4x4::rotation_from_quaternion(self.rotation);
    Vector3::new(m[2][0], m[2][1], m[2][2]).normalized()
  }

  pub fn up(&self) -> Vector3 {
    let m = Matrix4x4::rotation_from_quaternion(self.rotation);
    Vector3::new(m[1][0], m[1][1], m[1][2]).normalized()
  }

  pub fn right(&self) -> Vector3 {
    let m = Matrix4x4::rotation_from_quaternion(self.rotation);
    Vector3::new(m[0][0], m[0][1], m[0][2]).normalized()
  }
}

impl Default for Transform {
  fn default() -> Self {
    Transform {
      position: Vector3::default(),
      rotation: Quaternion::default(),
      scale: Vector3::one(),
      world_matrix: Matrix4x4::default(),
      is_dirty: true,
    }
  }
}
