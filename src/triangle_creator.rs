use gl::types::*;
use crate::game_state::GameState;
use crate::model_creator::add_model;

pub fn add_triangle(game_state: &mut GameState) -> Option<()> {
  add_model(game_state, get_triangle_vertices())
}

fn get_triangle_vertices() -> Vec<GLfloat> {
  let vertices: Vec<GLfloat> = vec![
    // X    Y   Z       R     G     B   A
     0.0,  0.5, 0.0,    1.0, 0.0, 0.0, 1.0,
    -0.5, -0.5, 0.0,    0.0, 1.0, 0.0, 1.0,
     0.5, -0.5, 0.0,    0.0, 0.0, 1.0, 1.0,
  ];
  vertices
}