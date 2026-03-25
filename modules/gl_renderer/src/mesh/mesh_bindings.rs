use glwn::gl::Gl;
use math::{Vector2, Vector3};
use rendering::Mesh;

fn bind_mesh_vertex_array(gl: &Gl, mesh: &mut Mesh) {
  unsafe {
    // @todo - perf test
    // does the GenVertexArrays need to run every time the mesh is bound?
    // How much of an impact does it have to render times?

    gl.GenVertexArrays(1, &mut mesh.vao);
    gl.BindVertexArray(mesh.vao);
  }
}

fn bind_mesh_vertex_buffers(gl: &Gl, mesh: &mut Mesh) {
  unsafe {
    gl.GenBuffers(1, &mut mesh.vbo);
    gl.BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
    gl.BufferData(
      gl::ARRAY_BUFFER,
      (mesh.vertices.len() * std::mem::size_of::<Vector3>()) as isize,
      mesh.vertices.as_ptr() as *const _,
      gl::STATIC_DRAW,
    );
    gl.EnableVertexAttribArray(0);
    gl.VertexAttribPointer(
      0,
      3,
      gl::FLOAT,
      gl::FALSE,
      std::mem::size_of::<Vector3>() as i32,
      std::ptr::null(),
    );
  }
}

fn bind_mesh_uv_buffers(gl: &Gl, mesh: &mut Mesh) {
  unsafe {
    if !mesh.uvs.is_empty() {
      gl.GenBuffers(1, &mut mesh.uvbo);
      gl.BindBuffer(gl::ARRAY_BUFFER, mesh.uvbo);
      gl.BufferData(
        gl::ARRAY_BUFFER,
        (mesh.uvs.len() * std::mem::size_of::<Vector2>()) as isize,
        mesh.uvs.as_ptr() as *const _,
        gl::STATIC_DRAW,
      );
      gl.EnableVertexAttribArray(1);
      gl.VertexAttribPointer(
        1,
        2,
        gl::FLOAT,
        gl::FALSE,
        std::mem::size_of::<Vector2>() as i32,
        std::ptr::null(),
      );
    }
  }
}

fn bind_mesh_normal_buffers(gl: &Gl, mesh: &mut Mesh) {
  unsafe {
    if !mesh.normals.is_empty() {
      gl.GenBuffers(1, &mut mesh.nbo);
      gl.BindBuffer(gl::ARRAY_BUFFER, mesh.nbo);
      gl.BufferData(
        gl::ARRAY_BUFFER,
        (mesh.normals.len() * std::mem::size_of::<Vector3>()) as isize,
        mesh.normals.as_ptr() as *const _,
        gl::STATIC_DRAW,
      );
      gl.EnableVertexAttribArray(2);
      gl.VertexAttribPointer(
        2,
        3,
        gl::FLOAT,
        gl::FALSE,
        std::mem::size_of::<Vector3>() as i32,
        std::ptr::null(),
      );
    }
  }
}

fn bind_mesh_index_buffer(gl: &Gl, mesh: &mut Mesh) {
  unsafe {
    gl.GenBuffers(1, &mut mesh.ibo);
    gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ibo);
    gl.BufferData(
      gl::ELEMENT_ARRAY_BUFFER,
      (mesh.triangles.len() * std::mem::size_of::<u32>()) as isize,
      mesh.triangles.as_ptr() as *const _,
      gl::STATIC_DRAW,
    );
  }
}

pub fn bind_mesh_buffers(gl: &Gl, mesh: &mut Mesh) {
  unsafe {
    bind_mesh_vertex_array(gl, mesh);
    bind_mesh_vertex_buffers(gl, mesh);
    bind_mesh_uv_buffers(gl, mesh);
    bind_mesh_normal_buffers(gl, mesh);
    bind_mesh_index_buffer(gl, mesh);

    gl.BindVertexArray(0);
  }

  mesh.has_changed = false;
}