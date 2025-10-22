#![allow(dead_code)]
use std::collections::HashMap;

use crate::engine::{MaterialId, ProgramId};

pub enum UniformValue {
  Float(f32),
  Vec3([f32; 3]),
  Mat3([[f32; 3]; 3]),
  Mat4([[f32; 4]; 4]),
}

pub struct Material {
  id: Option<MaterialId>,
  program_id: ProgramId,
  pub uniforms: HashMap<String, UniformValue>,
}

impl Material {
  pub fn new(program_id: ProgramId) -> Self {
    Material {
      id: None,
      program_id,
      uniforms: HashMap::new(),
    }
  }

  pub fn id(&self) -> Option<MaterialId> {
    self.id
  }

  pub fn program_id(&self) -> &ProgramId {
    &self.program_id
  }

  pub fn set_id(&mut self, id: MaterialId) -> &mut Self {
    self.id = Some(id);
    self
  }

  pub fn set_uniform(&mut self, name: &str, value: UniformValue) {
    self.uniforms.insert(name.to_string(), value);
  }
}
