use crate::Matrix4x4;

#[derive(Clone, Copy)]
pub struct Quaternion {
  pub w: f32,
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Quaternion {
  pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
    Quaternion { w, x, y, z }
  }

  pub fn identity() -> Self {
    Self {
      w: 1.0,
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }

  #[inline]
  pub fn from_axis_angle(axis: (f32, f32, f32), radians: f32) -> Self {
    let (x, y, z) = axis;
    let half = radians * 0.5;
    let (s, c) = (half.sin(), half.cos());
    Self {
      w: c,
      x: x * s,
      y: y * s,
      z: z * s,
    }
  }

  #[inline]
  pub fn multiply(&self, other: &Self) -> Self {
    Self {
      w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
      x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
      y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
      z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
    }
  }

  pub fn conjugate(&self) -> Self {
    Self {
      w: self.w,
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }

  pub fn rotate_yaw(&mut self, radians: f32) -> &mut Self {
    let q = Quaternion::from_axis_angle((0.0, 1.0, 0.0), radians);
    *self = q.multiply(self);
    self
  }

  pub fn rotate_pitch(&mut self, radians: f32) -> &mut Self {
    let q = Quaternion::from_axis_angle((1.0, 0.0, 0.0), radians);
    *self = q.multiply(self);
    self
  }

  pub fn rotate_roll(&mut self, radians: f32) -> &mut Self {
    let q = Quaternion::from_axis_angle((0.0, 0.0, 1.0), radians);
    *self = q.multiply(self);
    self
  }

  pub fn normalize(&mut self) -> &mut Self {
    let len = (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    if len > 0.0 {
      self.w /= len;
      self.x /= len;
      self.y /= len;
      self.z /= len;
    }
    self
  }
}

impl Default for Quaternion {
  fn default() -> Self {
    Quaternion::identity()
  }
}
