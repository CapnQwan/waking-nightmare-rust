use crate::math::Vector3;

#[rustfmt::skip]
pub const PLANE_VERTEX_DATA: [f32; 12] = [
	-0.5, 0.5, 0.0,
	0.5,  0.5, 0.0,
	0.5, -0.5, 0.0,
	-0.5, -0.5, 0.0,
];

#[rustfmt::skip]
pub const PLANE_VERTICIES: [Vector3; 4]  = [
  Vector3 {x: -0.5, y: 0.5,  z: 0.0},
  Vector3 {x: 0.5,  y: 0.5,  z: 0.0},
  Vector3 {x: 0.5,  y: -0.5, z: 0.0},
  Vector3 {x: -0.5, y: -0.5, z: 0.0},
];

#[rustfmt::skip]
pub const PLANE_TRIANGLES: [u32; 6]  = [
  0, 1, 2,
	0, 3, 2,
];
