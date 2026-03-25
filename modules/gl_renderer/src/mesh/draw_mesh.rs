use glwn::gl::Gl;
use rendering::Mesh;

pub fn draw_mesh(gl: &Gl, mesh: &Mesh) {
  unsafe {
    gl.BindVertexArray(mesh.vao);
    gl.DrawElements(
      gl::TRIANGLES,
      mesh.triangles.len() as i32,
      gl::UNSIGNED_INT,
      std::ptr::null(),
    );
    gl.BindVertexArray(0);
  }
}