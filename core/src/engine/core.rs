use std::sync::Arc;

use glwn::gl::Gl;
use log::info;

use crate::{
  assets::{CUBE_TRIANGLES, CUBE_VERTICIES, FRAGMENT_SHADER_SOURCE, VERTEX_SHADER_SOURCE},
  engine::{
    Material, Mesh, Program, RenderComponent, Renderer, Systems, Time, World, mesh_render_system,
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
    let object = world.spawn_object();
    let (components, resources) = world.split_borrow();

    resources.add_resource(Time::new());
    resources.add_resource(Renderer::new(gl));

    let renderer = resources.get_mut_resource::<Renderer>().unwrap();

    let program = {
      let glum_program = renderer
        .program_renderer_mut()
        .create_gl_program(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);
      Program::new(glum_program)
    };
    let program_id = { renderer.program_registry_mut().register(program) };

    let mut mesh = Mesh::new();
    mesh
      .set_vertices(CUBE_VERTICIES.to_vec())
      .set_triangles(CUBE_TRIANGLES.to_vec());
    let mesh_id = { renderer.mesh_registry_mut().register(mesh) };

    let material = Material::new(program_id);
    let material_id = { renderer.material_registry_mut().register(material) };

    let render_component = RenderComponent::new(mesh_id, material_id);
    components.add_component(object, render_component);

    let mut systems = Systems::new();
    systems.add_system(mesh_render_system);

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
