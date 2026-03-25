use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ShaderUniform {
  pub name: String,
  pub location: i32,
}

#[derive(Debug, Clone)]
pub struct ShaderAttribute {
  pub name: String,
  pub location: i32,
}

// @todo - can this logic just be moved out and into materials?

#[derive(Debug, Clone)]
pub struct ShaderReflection {
  pub uniforms: HashMap<String, ShaderUniform>,
  pub attributes: HashMap<String, ShaderAttribute>,
}
