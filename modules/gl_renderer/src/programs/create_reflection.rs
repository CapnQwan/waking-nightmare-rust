use std::collections::HashMap;
use std::ffi::CString;
use std::sync::Arc;
use glwn::gl::Gl;
use crate::{ShaderAttribute, ShaderReflection, ShaderUniform};

pub fn create_program_reflection(gl: Arc<Gl>, program: u32) -> ShaderReflection {
  unsafe {
    let mut reflection = ShaderReflection {
      uniforms: HashMap::new(),
      attributes: HashMap::new(),
    };

    // ----- Uniforms -----
    let mut uniform_count = 0;
    gl.GetProgramiv(program, gl::ACTIVE_UNIFORMS, &mut uniform_count);
    for i in 0..uniform_count {
      let mut name_buf = [0u8; 256];
      let mut length = 0;
      let mut size = 0;
      let mut utype = 0;
      gl.GetActiveUniform(
        program,
        i as u32,
        name_buf.len() as i32,
        &mut length,
        &mut size,
        &mut utype,
        name_buf.as_mut_ptr() as *mut _,
      );
      let name = String::from_utf8_lossy(&name_buf[..length as usize]).to_string();
      let cname = CString::new(name.clone()).unwrap();
      let location = gl.GetUniformLocation(program, cname.as_ptr());
      reflection
        .uniforms
        .insert(name.clone(), ShaderUniform { name, location });
    }

    // ----- Attributes -----
    let mut attrib_count = 0;
    gl.GetProgramiv(program, gl::ACTIVE_ATTRIBUTES, &mut attrib_count);
    for i in 0..attrib_count {
      let mut name_buf = [0u8; 256];
      let mut length = 0;
      let mut size = 0;
      let mut atype = 0;
      gl.GetActiveAttrib(
        program,
        i as u32,
        name_buf.len() as i32,
        &mut length,
        &mut size,
        &mut atype,
        name_buf.as_mut_ptr() as *mut _,
      );
      let name = String::from_utf8_lossy(&name_buf[..length as usize]).to_string();
      let cname = CString::new(name.clone()).unwrap();
      let location = gl.GetAttribLocation(program, cname.as_ptr());
      reflection
        .attributes
        .insert(name.clone(), ShaderAttribute { name, location });
    }

    reflection
  }
}