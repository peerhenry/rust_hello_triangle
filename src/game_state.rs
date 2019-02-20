use crate::shader_program::{ ShaderProgram, SetUniform };
use gl::types::*;
use cgmath::{ Matrix4 };
use crate::camera::Camera;
use engine::ecs::generational_index::*;
use engine::ecs::generational_entries::*;

// GameState

#[derive(Default)]
pub struct GameState {
  pub running: bool,
  pub shader_program: Option<ShaderProgram>,
  pub camera: Option<Camera>,
  // ECS
  pub entity_allocator: GenerationalIndexAllocator,
  pub vaos: GenerationalEntries<GLuint>,
  pub model_matrices: GenerationalEntries<Matrix4<GLfloat>>,
  pub vertex_counts: GenerationalEntries<GLsizei>,
  pub entities: Vec<GenerationalIndex>
}

impl GameState {
  pub fn new(shader_program: Option<ShaderProgram>) -> GameState {
    GameState {
      running: true,
      shader_program,
      ..Default::default()
    }
  }

  pub fn draw(&self) -> Result<(),&str> {
    unsafe { gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT); }
    let program = self.shader_program.as_ref().map_or(Err("Trying to draw but no shader program in GameState"), |p| Ok(p))?;
    let cam = self.camera.as_ref().map_or(Err("Trying to draw but no camera in GameState"), |c| Ok(c))?;
    unsafe {
      program.set_uniform_matrix("View", cam.view_matrix);
      program.set_uniform_matrix("Projection", cam.projection_matrix);
    }
    for entity_index in &self.entities {
      self.draw_entity(program, *entity_index);
    }
    Ok(())
  }

  fn draw_entity(&self, program: &ShaderProgram, entity_index: GenerationalIndex) -> Option<()> {
    let vao = *self.vaos.get(entity_index)?;
    let model_matrix = *self.model_matrices.get(entity_index)?;
    let vertex_count = *self.vertex_counts.get(entity_index)?;
    unsafe {
      program.set_uniform_matrix("Model", model_matrix);
      gl::BindVertexArray(vao);
      gl::DrawArrays(gl::TRIANGLES, 0, vertex_count);
    }
    Some(())
  }
}

// builder
#[derive(Default)]
pub struct GameStateBuilder {
  pub shader_program: Option<ShaderProgram>,
  pub camera: Option<Camera>,
}

impl GameStateBuilder {
  #[allow(dead_code)]
  pub fn new() -> Self {
    Default::default()
  }

  #[allow(dead_code)]
  pub fn with_shader_program(mut self, shader_program: Option<ShaderProgram>) -> Self {
    self.shader_program = shader_program;
    self
  }

  #[allow(dead_code)]
  pub fn with_camera(mut self, camera: Option<Camera>) -> Self {
    self.camera = camera;
    self
  }

  #[allow(dead_code)]
  pub fn build(self) -> GameState {
    let mut state = GameState::new(self.shader_program);
    state.camera = self.camera;
    state
  }
}

#[cfg(test)]
mod game_state_tests {
  use super::*;

  #[test]
  fn can_build_empty_game_state() {
    // arrange
    let builder = GameStateBuilder::new();
    // act
    let game = builder.build();
    // assert
    assert!(game.camera.is_none());
    assert!(game.shader_program.is_none());
  }
}