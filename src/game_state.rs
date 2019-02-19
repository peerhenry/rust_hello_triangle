use crate::shader_program::ShaderProgram;
use gl::types::*;
use cgmath::{ Matrix4 };
use crate::camera::Camera;

#[derive(Default)]
pub struct GameState {
  // assets
  pub running: bool,
  pub shader_program: Option<ShaderProgram>,
  pub camera: Option<Camera>,
  // components
  pub vaos: Vec<GLuint>,
  pub model_matrices: Vec<Matrix4<GLfloat>>,
  // entity indices
  pub entities: Vec<usize>
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
    // todo: move to GameState
    for entity_index in &self.entities {
      let vao = self.vaos[*entity_index];
      let model_matrix = self.model_matrices[*entity_index];
      unsafe {
        program.set_uniform_matrix("Model", model_matrix);
        gl::BindVertexArray(vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
      }
    }
    Ok(())
  }
}