use std::mem::size_of;
use gl::types::*;
use cgmath::{ Matrix4, SquareMatrix };
use GameState;
use VaoBuilder;
use AttribParameters;

pub fn add_triangle(game: &mut GameState) {
  // buffers
  let buffers = VaoBuilder::new()
    .with_attribute(AttribParameters{ // position
      floats_per_attribute: 3,
      floats_per_vertex: 7,
      offset: 0
    })
    .with_attribute(AttribParameters{ // color
      floats_per_attribute: 4,
      floats_per_vertex: 7,
      offset: 3
    })
    .build();
  unsafe { populate_vbo(buffers.vbo) };
  let vao = buffers.vao;
  game.vaos.push(vao);
  // model matrix
  let model_matrix: Matrix4<GLfloat> = Matrix4::from_value(1.0);
  game.model_matrices.push(model_matrix);
  game.entities.push(0);
}

unsafe fn populate_vbo(vbo: GLuint) {
  // ##  Setup vertex data
  let vertices: Vec<GLfloat> = vec![
    // X    Y   Z       R     G     B   A
     0.0,  0.5, 0.0,    1.0, 0.0, 0.0, 1.0,
    -0.5, -0.5, 0.0,    0.0, 1.0, 0.0, 1.0,
     0.5, -0.5, 0.0,    0.0, 0.0, 1.0, 1.0
  ];
  gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
  gl::BufferData(
    gl::ARRAY_BUFFER,                                       // target
    (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,  // size in bytes
    vertices.as_ptr() as *const GLvoid,                     // data
    gl::STATIC_DRAW                                         // usage
  );
  gl::BindBuffer(gl::ARRAY_BUFFER, 0);
}
