use engine::vao_builder::buffer_component::BufferComponent;
use std::mem::size_of;
use gl::types::*;
use cgmath::{ Matrix4, SquareMatrix };
use crate::game_state::GameState;
use engine::vao_builder::VaoBuilder;
use engine::vao_builder::attrib_parameters::AttribParameters;

pub fn add_model(game_state: &mut GameState, vertices: Vec<GLfloat>) -> Option<()> {
  let (buffers, floats_per_vertex) = build_buffers(game_state);
  let vertex_count = unsafe { populate_vbo(buffers.vbo, floats_per_vertex, vertices) };
  add_to_game(buffers, game_state, vertex_count);
  Some(())
}

fn build_buffers(game_state: &GameState) -> (BufferComponent, usize) {
  // todo: get attributes from program (floats per attribute, floats per vertex)
  // game_state.program?.get_active_attributes();
  let floats_per_vertex: usize = 7;
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
  (buffers, floats_per_vertex)
}

unsafe fn populate_vbo(vbo: GLuint, floats_per_vertex: usize, vertices: Vec<GLfloat>) -> GLsizei {
  // ##  Setup vertex data
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

fn add_to_game(buffers: BufferComponent, game_state: &mut GameState, vertex_count: GLsizei) {
  let vao = buffers.vao;
  let model_matrix: Matrix4<GLfloat> = Matrix4::from_value(1.0);
  // add entity to game; todo: use allocator
  let generational_index = game_state.entity_allocator.allocate();
  game_state.vaos.set(generational_index, vao);
  game_state.model_matrices.set(generational_index, model_matrix);
  game_state.vertex_counts.set(generational_index, vertex_count);
  game_state.entities.push(generational_index);
}