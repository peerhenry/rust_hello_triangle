use std::mem::size_of;
use gl::types::*;
use cgmath::{ Matrix4, SquareMatrix };
use crate::game_state::GameState;
use engine::vao_builder::VaoBuilder;
use engine::vao_builder::attrib_parameters::AttribParameters;

pub fn add_triangle(game: &mut GameState) {
  let floats_per_vertex: usize = 7;
  // buffers
  let buffers = VaoBuilder::new()
    .with_attribute(AttribParameters{ // position
      floats_per_attribute: 3,
      floats_per_vertex,
      offset: 0
    })
    .with_attribute(AttribParameters{ // color
      floats_per_attribute: 4,
      floats_per_vertex,
      offset: 3
    })
    .build();
  let vertex_count = unsafe { populate_vbo(buffers.vbo, floats_per_vertex) };
  let vao = buffers.vao;
  let model_matrix: Matrix4<GLfloat> = Matrix4::from_value(1.0);
  // add entity to game; todo: use allocator
  game.vaos.push(vao);
  game.model_matrices.push(model_matrix);
  game.vertex_counts.push(vertex_count);
  game.entities.push(0);
}

unsafe fn populate_vbo(vbo: GLuint, floats_per_vertex: usize) -> GLsizei {
  // ##  Setup vertex data
  let vertices: Vec<GLfloat> = vec![
    // X    Y   Z       R     G     B   A
     0.0,  0.5, 0.0,    1.0, 0.0, 0.0, 1.0,
    -0.5, -0.5, 0.0,    0.0, 1.0, 0.0, 1.0,
     0.5, -0.5, 0.0,    0.0, 0.0, 1.0, 1.0,

    /*0.0,  0.1, 0.1,    1.0, 0.0, 0.0, 1.0, // uncomment to see a smaller triangle in front of the first
    -0.1, -0.1, 0.1,    0.0, 1.0, 0.0, 1.0,
     0.1, -0.1, 0.1,    0.0, 0.0, 1.0, 1.0*/
  ];
  gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
  gl::BufferData(
    gl::ARRAY_BUFFER,                                       // target
    (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,  // size in bytes
    vertices.as_ptr() as *const GLvoid,                     // data
    gl::STATIC_DRAW                                         // usage
  );
  gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  return (vertices.len()/floats_per_vertex) as _;
}
