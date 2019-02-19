use crate::shader_program::ShaderProgram;
use gl::types::*;
use cgmath::{ Matrix4 };
use crate::camera::Camera;

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
      camera: None,
      vaos: Vec::new(),
      model_matrices: Vec::new(),
      entities: Vec::new()
    }
  }
}