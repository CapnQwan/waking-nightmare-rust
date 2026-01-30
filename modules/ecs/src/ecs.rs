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

pub struct ECS {
    world: World,
    systems: Systems,
}

impl ECS {
    pub fn new() -> Self {
        let world = World::new();
        let systems = Systems::new();

        ECS { world, systems }
    }

    pub fn update(&mut self) {
        self.world.update_resources();
        self.systems.update(&mut self.world);
    }
}
