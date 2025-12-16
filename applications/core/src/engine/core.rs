use std::sync::Arc;

use glwn::gl::Gl;
use math::{Transform, Vector3};

use crate::{
  assets::{CUBE_TRIANGLES, CUBE_VERTICIES, LIT_FRAGMENT_SHADER_SOURCE, LIT_VERTEX_SHADER_SOURCE},
  engine::{
    Camera, Material, Mesh, Program, RenderComponent, Renderer, Systems, Time, World,
    camera_view_projection_system, mesh_render_system, temp_example_system,
  },
  traits::Registry,
};

pub struct Core {
  world: World,
  systems: Systems,
}

// @Todo
// Move all component / entity / system binding to seperate functions.
// Add logic for parsing and saving worlds to some format. Maybe just
// storing all the component data as .rs might be easiest and fastest?
// Need to test perf

impl Core {
  pub fn new(gl: Arc<Gl>) -> Self {
    let mut world = World::new();
    // let camera = world.spawn_object();
    // let object = world.spawn_object();
    let (_, resources) = world.split_borrow();

    resources.add_resource(Time::new());
    resources.add_resource(Renderer::new(gl));

    let mut systems = Systems::new();
    systems.add_system(mesh_render_system);
    systems.add_system(camera_view_projection_system);
    systems.add_system(temp_example_system);

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
      renderer.draw();
    }
  }
}
