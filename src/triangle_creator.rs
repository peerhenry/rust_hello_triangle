use std::mem::size_of;
use gl::types::*;
use cgmath::{ Matrix4, SquareMatrix };
use GameState;

pub fn add_triangle(game: &mut GameState) {
  let vbo = unsafe { setup_vbo() };
  let vao = unsafe { setup_vao(vbo) };
  game.vaos.push(vao);
  let model_matrix: Matrix4<GLfloat> = Matrix4::from_value(1.0);
  game.model_matrices.push(model_matrix);
  game.entities.push(0);
}

unsafe fn setup_vbo() -> GLuint {
  // ##  Setup vertex data
  let vertices: Vec<GLfloat> = vec![
    // X    Y   Z       R     G     B   A
     0.0,  0.5, 0.0,    1.0, 0.0, 0.0, 1.0,
    -0.5, -0.5, 0.0,    0.0, 1.0, 0.0, 1.0,
     0.5, -0.5, 0.0,    0.0, 0.0, 1.0, 1.0
  ];
  let mut vbo: GLuint = 0;
  gl::GenBuffers(1, &mut vbo);
  gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
  gl::BufferData(
    gl::ARRAY_BUFFER,                                       // target
    (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,  // size in bytes
    vertices.as_ptr() as *const GLvoid,                     // data
    gl::STATIC_DRAW                                         // usage
  );
  gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  vbo
}

unsafe fn setup_vao(vbo: GLuint) -> GLuint {
  let mut vao: GLuint = 0;
  gl::GenVertexArrays(1, &mut vao); 
  gl::BindVertexArray(vao);
  gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

  // location, f/attribute, f/vertex (stride), offset
  setup_attribute(0, 3, 7, 0);  // position
  setup_attribute(1, 4, 7, 3);  // color

  gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  gl::BindVertexArray(0);
  vao
}

unsafe fn setup_attribute(location: GLuint, floats_per_attribute: GLint, floats_per_vertex: usize, offset: usize){
  gl::EnableVertexAttribArray(location); // this is "layout (location = 0)" in vertex shader
  gl::VertexAttribPointer(
    location,   // location
    floats_per_attribute,          // number per attribute
    gl::FLOAT,  // data type
    gl::FALSE,  // normalized
    (floats_per_vertex * size_of::<GLfloat>()) as GLint,  // stride
    (offset * size_of::<GLfloat>()) as *const gl::types::GLvoid  // offset
  );
}