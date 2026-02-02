use std::collections::HashMap;
use crate::{MaterialId, ProgramId};

pub enum UniformValue {
  Float(f32),
  Vec3([f32; 3]),
  Mat3([[f32; 3]; 3]),
  Mat4([[f32; 4]; 4]),
}

pub struct Material {
  id: MaterialId,
  program_id: ProgramId,
  pub uniforms: HashMap<String, UniformValue>,
}

impl Material {
  pub fn new(program_id: ProgramId, material_id: MaterialId) -> Self {
    Material {
      id: material_id,
      program_id,
      uniforms: HashMap::new(),
    }
  }

  pub fn id(&self) -> MaterialId {
    self.id
  }

  pub fn program_id(&self) -> &ProgramId {
    &self.program_id
  }

  pub fn set_uniform(&mut self, name: &str, value: UniformValue) {
    self.uniforms.insert(name.to_string(), value);
  }
}
