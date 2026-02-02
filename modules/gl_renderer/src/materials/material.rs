use std::collections::HashMap;
use crate::{ShaderAttribute, ShaderUniform};

pub struct Material {
  pub program_id: u32,
  pub uniforms: HashMap<String, ShaderUniform>,
  pub attributes: HashMap<String, ShaderAttribute>,
}
