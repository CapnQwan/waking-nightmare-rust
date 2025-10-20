use std::{collections::HashMap, ffi::CString, sync::Arc};

use glwn::gl::Gl;

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

#[derive(Debug, Clone)]
pub struct ShaderReflection {
  pub uniforms: HashMap<String, ShaderUniform>,
  pub attributes: HashMap<String, ShaderAttribute>,
}

pub struct ProgramRenderer {
  gl: Arc<Gl>,
}

impl ProgramRenderer {
  pub fn new(gl: Arc<Gl>) -> Self {
    Self { gl }
  }

  pub fn bind_program() {}

  pub fn bind_attributes() {}

  pub fn bind_uniforms() {}

  pub fn bind_textures() {}

  pub unsafe fn create_shader(
    &self,
    shader: gl::types::GLenum,
    source: &[u8],
  ) -> gl::types::GLuint {
    unsafe {
      let shader = self.gl.CreateShader(shader);
      self.gl.ShaderSource(
        shader,
        1,
        [source.as_ptr().cast()].as_ptr(),
        std::ptr::null(),
      );
      self.gl.CompileShader(shader);
      shader
    }
  }

  pub fn create_gl_program(
    &self,
    vertext_shader_source: &[u8],
    fragment_shader_source: &[u8],
  ) -> u32 {
    unsafe {
      let vertex_shader = self.create_shader(gl::VERTEX_SHADER, vertext_shader_source);
      self.check_shader_compile(vertex_shader);
      let fragment_shader = self.create_shader(gl::FRAGMENT_SHADER, fragment_shader_source);
      self.check_shader_compile(fragment_shader);

      let program = self.gl.CreateProgram();

      self.gl.AttachShader(program, vertex_shader);
      self.gl.AttachShader(program, fragment_shader);

      self.gl.LinkProgram(program);

      self.check_program_link(program);

      self.gl.UseProgram(program);

      self.gl.DeleteShader(vertex_shader);
      self.gl.DeleteShader(fragment_shader);

      program
    }
  }

  pub fn reflect_program(&self, program: u32) -> ShaderReflection {
    unsafe {
      let mut reflection = ShaderReflection {
        uniforms: HashMap::new(),
        attributes: HashMap::new(),
      };

      // ----- Uniforms -----
      let mut uniform_count = 0;
      self
        .gl
        .GetProgramiv(program, gl::ACTIVE_UNIFORMS, &mut uniform_count);
      for i in 0..uniform_count {
        let mut name_buf = [0u8; 256];
        let mut length = 0;
        let mut size = 0;
        let mut utype = 0;
        self.gl.GetActiveUniform(
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
        let location = self.gl.GetUniformLocation(program, cname.as_ptr());
        reflection
          .uniforms
          .insert(name.clone(), ShaderUniform { name, location });
      }

      // ----- Attributes -----
      let mut attrib_count = 0;
      self
        .gl
        .GetProgramiv(program, gl::ACTIVE_ATTRIBUTES, &mut attrib_count);
      for i in 0..attrib_count {
        let mut name_buf = [0u8; 256];
        let mut length = 0;
        let mut size = 0;
        let mut atype = 0;
        self.gl.GetActiveAttrib(
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
        let location = self.gl.GetAttribLocation(program, cname.as_ptr());
        reflection
          .attributes
          .insert(name.clone(), ShaderAttribute { name, location });
      }

      reflection
    }
  }

  unsafe fn check_shader_compile(&self, shader: u32) {
    unsafe {
      let mut status = 0;
      self.gl.GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
      if status == 0 {
        let mut len = 0;
        self.gl.GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

        let mut buffer = vec![0u8; len as usize];
        self.gl.GetShaderInfoLog(
          shader,
          len,
          std::ptr::null_mut(),
          buffer.as_mut_ptr() as *mut _,
        );

        let log = String::from_utf8_lossy(&buffer);
        log::error!("Shader compile failed:\n{}", log);
      } else {
        log::info!("Shader compiled successfully.");
      }
    }
  }

  unsafe fn check_program_link(&self, program: u32) {
    unsafe {
      let mut status = 0;
      self.gl.GetProgramiv(program, gl::LINK_STATUS, &mut status);
      if status == 0 {
        let mut len = 0;
        self.gl.GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

        let mut buffer = vec![0u8; len as usize];
        self.gl.GetProgramInfoLog(
          program,
          len,
          std::ptr::null_mut(),
          buffer.as_mut_ptr() as *mut _,
        );

        let log = String::from_utf8_lossy(&buffer);
        log::error!("Program link failed:\n{}", log);
      } else {
        log::info!("Program linked successfully.");
      }
    }
  }
}
