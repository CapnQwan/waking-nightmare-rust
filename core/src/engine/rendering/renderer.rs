use std::{ffi::CStr, sync::Arc};

use crate::{
  assets::{CUBE_TRIANGLES, CUBE_VERTICIES, LIT_FRAGMENT_SHADER_SOURCE, LIT_VERTEX_SHADER_SOURCE},
  engine::{
    Camera, Material, MaterialId, MaterialRegistry, MaterialRenderer, Mesh, MeshId, MeshRegistry,
    MeshRenderer, Program, ProgramRegistry, ProgramRenderer, RenderComponent, UniformValue,
  },
  traits::Registry,
};
use glwn::gl::Gl;
use math::{Matrix4x4, Transform, Vector3};

#[derive(Clone, Copy)]
pub struct RenderCommand {
  mesh_id: MeshId,
  material_id: MaterialId,
  transform: Transform,
  camera_position: Vector3,
  view_matrix: Matrix4x4,
  projection_matrix: Matrix4x4,
}

/** @todo - update so camera_id is passed instead opting for retrieving the camera from the ECS */
impl RenderCommand {
  pub fn new(
    mesh_id: MeshId,
    material_id: MaterialId,
    transform: Transform,
    camera_position: Vector3,
    view_matrix: Matrix4x4,
    projection_matrix: Matrix4x4,
  ) -> Self {
    RenderCommand {
      mesh_id,
      material_id,
      transform,
      camera_position,
      view_matrix,
      projection_matrix,
    }
  }
}

/** @todo
 * would it be better to switch the registries and maybe even the renderers to
 * be a hash map with a look up or something like that?
 *
 * Maybe this would be better depending on how the engine continues to develop
 * for example if a texture registry or other registries end up being added this
 * idea might be better.
 *
 * Maybe even seperating this logic off so that all Assets are stored in a seperate struct
 * might be better that way other non redering assets like audio can be stored all in one clean
 * little abstraction
 */
pub struct Renderer {
  gl: Arc<Gl>,
  material_renderer: MaterialRenderer,
  material_registry: MaterialRegistry,
  mesh_renderer: MeshRenderer,
  mesh_registry: MeshRegistry,
  program_renderer: ProgramRenderer,
  program_registry: ProgramRegistry,
  queued_render_calls: Vec<RenderCommand>,
}

impl Renderer {
  pub fn new(gl: Arc<Gl>) -> Self {
    let material_renderer = MaterialRenderer::new(gl.clone());
    let mesh_renderer = MeshRenderer::new(gl.clone());
    let program_renderer = ProgramRenderer::new(gl.clone());

    Self {
      gl,
      material_renderer,
      mesh_renderer,
      program_renderer,
      material_registry: MaterialRegistry::new(),
      mesh_registry: MeshRegistry::new(),
      program_registry: ProgramRegistry::new(),
      queued_render_calls: Vec::new(),
    }
  }

  pub fn clear(&self) -> &Self {
    unsafe {
      self.gl.ClearColor(1.0, 1.0, 1.0, 1.0);
      self.gl.Clear(gl::COLOR_BUFFER_BIT);
      self
    }
  }

  pub fn queue_draw(&mut self, render_command: RenderCommand) {
    self.queued_render_calls.push(render_command);
  }

  pub fn draw(&mut self) {
    let program = {
      let glum_program = self
        .program_renderer_mut()
        .create_gl_program(LIT_VERTEX_SHADER_SOURCE, LIT_FRAGMENT_SHADER_SOURCE);
      Program::new(glum_program)
    };
    let program_id = { self.program_registry_mut().register(program) };

    let mut mesh = Mesh::new();
    mesh
      .set_vertices(CUBE_VERTICIES.to_vec())
      .set_triangles(CUBE_TRIANGLES.to_vec());
    //let mesh_id = { self.mesh_registry_mut().register(mesh) };

    let mut material = Material::new(program_id);
    //let material_id = { self.material_registry_mut().register(material) };

    //let render_component = RenderComponent::new(mesh_id, material_id);
    let mut render_component_transform = Transform::default();
    render_component_transform.update_world_matrix();

    let mut camera = Camera::default();
    let mut camera_transform = Transform::default();
    camera_transform.set_position(Vector3::new(0.0, 0.0, 5.0));
    camera_transform.update_world_matrix();
    camera.update_projection();
    camera.update_view_matrix(camera_transform);

    let model_matrix = render_component_transform.world_matrix();
    let view_matrix = camera.view_matrix().clone();
    let projection_matrix = camera.projection_matrix();

    let model_view = view_matrix * model_matrix;
    let normal_matrix = model_view.inverse().transpose().to_matrix3x3();

    // @todo - move bulk of this logic out of the draw execution

    // @todo - improve this as materials get developed more to be handled cleaner and more
    // effcient as some of this data doesn't need to be recalculated and re handled between
    // draw calls
    material.set_uniform(
      "uViewPosition",
      UniformValue::Vec3(camera_transform.position().to_array()),
    );
    material.set_uniform(
      "uModelMatrix",
      UniformValue::Mat4(model_matrix.as_column_major()),
    );
    material.set_uniform(
      "uViewMatrix",
      UniformValue::Mat4(view_matrix.as_column_major()),
    );
    material.set_uniform(
      "uProjectionMatrix",
      UniformValue::Mat4(projection_matrix.as_column_major()),
    );
    material.set_uniform(
      "uNormalMatrix",
      UniformValue::Mat3(normal_matrix.as_column_major()),
    );

    let program = self.program_registry.get(material.program_id()).unwrap();
    // @todo - add caching to reflections
    let reflection = self.program_renderer.reflect_program(program.program());
    self.material_renderer.bind_material(&material, &reflection);

    unsafe {
      self.gl.UseProgram(program.program());
    }

    if mesh.has_changed() {
      self.mesh_renderer.bind_mesh_buffers(&mut mesh);
    }
    self.mesh_renderer.draw_mesh(&mesh);
    // let queued_render_calls = self.queued_render_calls.clone();
    // for render_command in &queued_render_calls {
    //   self.execute_draw_command(render_command);
    // }
    // self.queued_render_calls.clear();
  }

  fn execute_draw_command(&mut self, render_command: &RenderCommand) {
    let (Some(material), Some(mesh)) = (
      self.material_registry.get_mut(&render_command.material_id),
      self.mesh_registry.get_mut(&render_command.mesh_id),
    ) else {
      return;
    };

    let model_matrix = render_command.transform.world_matrix();
    let view_matrix = render_command.view_matrix;
    let projection_matrix = render_command.projection_matrix;

    let model_view = view_matrix * model_matrix;
    let normal_matrix = model_view.inverse().transpose().to_matrix3x3();

    // @todo - move bulk of this logic out of the draw execution

    // @todo - improve this as materials get developed more to be handled cleaner and more
    // effcient as some of this data doesn't need to be recalculated and re handled between
    // draw calls
    material.set_uniform(
      "uViewPosition",
      UniformValue::Vec3(render_command.camera_position.to_array()),
    );
    material.set_uniform(
      "uModelMatrix",
      UniformValue::Mat4(model_matrix.as_column_major()),
    );
    material.set_uniform(
      "uViewMatrix",
      UniformValue::Mat4(view_matrix.as_column_major()),
    );
    material.set_uniform(
      "uProjectionMatrix",
      UniformValue::Mat4(projection_matrix.as_column_major()),
    );
    material.set_uniform(
      "uNormalMatrix",
      UniformValue::Mat3(normal_matrix.as_column_major()),
    );

    let program = self.program_registry.get(material.program_id()).unwrap();
    // @todo - add caching to reflections
    let reflection = self.program_renderer.reflect_program(program.program());
    self.material_renderer.bind_material(material, &reflection);

    unsafe {
      self.gl.UseProgram(program.program());
    }

    if mesh.has_changed() {
      self.mesh_renderer.bind_mesh_buffers(mesh);
    }
    self.mesh_renderer.draw_mesh(mesh);
  }

  pub fn resize(&self, width: i32, height: i32) {
    unsafe {
      self.gl.Viewport(0, 0, width, height);
    }
  }

  fn get_gl_string(&self, variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
      let s = self.gl.GetString(variant);
      (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
  }

  pub fn program_registry(&self) -> &ProgramRegistry {
    &self.program_registry
  }

  pub fn program_registry_mut(&mut self) -> &mut ProgramRegistry {
    &mut self.program_registry
  }

  pub fn program_renderer(&self) -> &ProgramRenderer {
    &self.program_renderer
  }

  pub fn program_renderer_mut(&mut self) -> &mut ProgramRenderer {
    &mut self.program_renderer
  }

  pub fn material_renderer(&self) -> &MaterialRenderer {
    &self.material_renderer
  }

  pub fn material_renderer_mut(&mut self) -> &mut MaterialRenderer {
    &mut self.material_renderer
  }

  pub fn material_registry(&self) -> &MaterialRegistry {
    &self.material_registry
  }

  pub fn material_registry_mut(&mut self) -> &mut MaterialRegistry {
    &mut self.material_registry
  }

  pub fn mesh_renderer(&self) -> &MeshRenderer {
    &self.mesh_renderer
  }

  pub fn mesh_renderer_mut(&mut self) -> &mut MeshRenderer {
    &mut self.mesh_renderer
  }

  pub fn mesh_registry(&self) -> &MeshRegistry {
    &self.mesh_registry
  }

  pub fn mesh_registry_mut(&mut self) -> &mut MeshRegistry {
    &mut self.mesh_registry
  }
}
