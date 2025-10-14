use std::sync::Arc;

use glwn::gl::Gl;

use crate::{
  assets::{
    CUBE_TRIANGLES, CUBE_VERTEX_DATA, CUBE_VERTICIES, FRAGMENT_SHADER_SOURCE, VERTEX_SHADER_SOURCE,
  },
  engine::{
    Material, MaterialId, Mesh, MeshId, Program, RenderComponent, Renderer, Systems, Time, World,
  },
  traits::Registry,
};

pub struct Core {
  world: World,
  systems: Systems,
}

impl Core {
  pub fn new(gl: Arc<Gl>) -> Self {
    let mut world = World::new();
    let (components, resources) = world.split_borrow();

    let renderer = Renderer::new(gl);

    resources.add_resource(Time::new());
    resources.add_resource(renderer);

    let object = world.spawn_object();

    let program_renderer = renderer.program_renderer();
    let program_registry = renderer.program_registry();
    let material_renderer = renderer.material_renderer();
    let material_registry = renderer.material_registry();
    let mesh_renderer = renderer.mesh_renderer();
    let mesh_registry = renderer.mesh_registry();

    let glum_program =
      program_renderer.create_gl_program(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);
    let program = Program::new(glum_program);
    let program_id = program_registry.register(program);
    let mesh = Mesh::new();
    mesh.verticies = CUBE_VERTICIES.to_vec();
    mesh.triangles = CUBE_TRIANGLES.to_vec();
    let mesh_id = mesh_registry.register(mesh);
    let material = Material::new(program);
    let material_id = material_registry.register(material);

    let render_component = RenderComponent::new(mesh_id, material_id);
    components.add_component(object, render_component);

    let systems = Systems::new();

    Core { world, systems }
  }

  pub fn update(&mut self) {
    self.world.update_resources();
    self.systems.update(&mut self.world);
  }

  pub fn draw(&mut self) {
    let (_, resources) = self.world.split_borrow();
    if let Some(renderer) = resources.get_mut_resource::<Renderer>() {
      renderer.clear();
    }
  }
}
