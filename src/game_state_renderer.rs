use crate::shader_program::{ ShaderProgram, SetUniform };
use gl::types::*;
// use cgmath::{ Matrix4 };
// use crate::camera::Camera;
use engine::ecs::generational_index::*;
// use engine::ecs::generational_entries::*;
use crate::game_state::GameState;

pub struct GameStateRenderer {
  mode: GLenum
}

impl GameStateRenderer {

  pub fn new(mode: GLenum) -> Self { // gl::TRIANGLES
    GameStateRenderer {
      mode
    }
  }

  pub fn draw(&self, game_state: &GameState) -> Result<(),&str> {
    unsafe { gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT); }
    let program = game_state.shader_program.as_ref().map_or(Err("Trying to draw but no shader program in GameState"), |p| Ok(p))?;
    let cam = game_state.camera.as_ref().map_or(Err("Trying to draw but no camera in GameState"), |c| Ok(c))?;
    unsafe {
      program.set_uniform_matrix("View", cam.view_matrix);
      program.set_uniform_matrix("Projection", cam.projection_matrix);
    }
    for entity_index in &game_state.entities {
      self.draw_entity(game_state, program, *entity_index);
    }
    Ok(())
  }

  fn draw_entity(&self, game_state: &GameState, program: &ShaderProgram, entity_index: GenerationalIndex) -> Option<()> {
    let vao = *game_state.vaos.get(entity_index)?;
    let model_matrix = *game_state.model_matrices.get(entity_index)?;
    let vertex_count = *game_state.vertex_counts.get(entity_index)?;
    unsafe {
      program.set_uniform_matrix("Model", model_matrix);
      gl::BindVertexArray(vao);
      gl::DrawArrays(self.mode, 0, vertex_count);
    }
    Some(())
  }
}