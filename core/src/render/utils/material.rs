use crate::render::utils::shader::Shader;

pub struct Material {
  pub shader: Shader,
  pub uniforms: u32, // @todo - not sure what type this should have yet so this is a placeholder
  pub attributes: u32, // @todo - not sure what type this should have yet so this is a placeholder
}

impl Material {
  pub fn utilize(&self) {}
  pub fn setUniform(&self) {}
  pub fn setAttributes(&self) {}
}
