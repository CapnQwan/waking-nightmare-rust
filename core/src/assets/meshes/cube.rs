use math::Vector3;

#[rustfmt::skip]
pub const CUBE_VERTEX_DATA: [f32; 24] = [
	-0.5, 0.5, -0.5,
	0.5,  0.5, -0.5,
	0.5, -0.5, -0.5,
	-0.5, -0.5, -0.5,
	-0.5, 0.5, 0.5,
	0.5,  0.5, 0.5,
	0.5, -0.5, 0.5,
	-0.5, -0.5, 0.5,
];

#[rustfmt::skip]
pub const CUBE_VERTICIES: [Vector3; 8]  = [
  Vector3 {x: -0.5, y: 0.5,  z: -0.5},
  Vector3 {x: 0.5,  y: 0.5,  z: -0.5},
  Vector3 {x: 0.5,  y: -0.5, z: -0.5},
  Vector3 {x: -0.5, y: -0.5, z: -0.5},
  Vector3 {x: -0.5, y: 0.5, z: 0.5},
  Vector3 {x: 0.5, y: 0.5, z: 0.5},
  Vector3 {x: 0.5, y: -0.5, z: 0.5},
  Vector3 {x: -0.5, y: -0.5, z: 0.5},
];

#[rustfmt::skip]
pub const CUBE_TRIANGLES: [u32; 36]  = [
  0, 1, 2,
  0, 3, 2,
  4, 6, 5,
  4, 7, 6,
  4, 0, 3,
  4, 3, 7,
  1, 5, 6,
  1, 6, 2,
  4, 5, 1,
  4, 1, 0,
  3, 2, 6,
  3, 6, 7,
];
