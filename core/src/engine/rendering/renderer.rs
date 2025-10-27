use std::{mem, sync::Arc};

use crate::{
  engine::{
    MaterialId, MaterialRegistry, MaterialRenderer, MeshId, MeshRegistry, MeshRenderer,
    ProgramRegistry, ProgramRenderer, UniformValue,
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

// @Todo
// update so camera_id is passed instead of all the different matricies
// opting for retrieving the camera from the ECS
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

// @Todo / Note
// would it be better to switch the registries and maybe even the renderers to
// be stored in some sort of registry to make development and itteration easier?
//
// Maybe this would be better depending on how the engine continues to develop
// for example if a texture registry or other registries end up being added this
// idea might be better.
//
// Maybe even seperating this logic off so that all Assets are stored in a seperate struct
// might be better that way other non redering assets like audio can be stored all in one clean
// little abstraction
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

  pub fn clear(&self) {
    unsafe {
      self.gl.ClearColor(0.2, 0.2, 0.2, 1.0);
      self.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
  }

  pub fn queue_draw(&mut self, render_command: RenderCommand) {
    self.queued_render_calls.push(render_command);
  }

  pub fn draw(&mut self) {
    let queued_render_calls = mem::take(&mut self.queued_render_calls);
    for render_command in &queued_render_calls {
      self.execute_draw_command(render_command);
    }
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

    // @Todo
    // move bulk of this logic out of the draw execution

    // @Todo
    // improve this as materials get developed more to be handled cleaner
    // and more effciently as some of this data doesn't need to be recalculated and
    // between draw calls
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
    unsafe {
      self.gl.UseProgram(program.program());
    }

    // @Note
    // add caching to reflections?
    let reflection = self.program_renderer.reflect_program(program.program());
    self.material_renderer.bind_material(material, &reflection);

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

  pub fn program_registry_mut(&mut self) -> &mut ProgramRegistry {
    &mut self.program_registry
  }

  pub fn program_renderer_mut(&mut self) -> &mut ProgramRenderer {
    &mut self.program_renderer
  }

  pub fn material_registry_mut(&mut self) -> &mut MaterialRegistry {
    &mut self.material_registry
  }

  pub fn mesh_registry_mut(&mut self) -> &mut MeshRegistry {
    &mut self.mesh_registry
  }
}
