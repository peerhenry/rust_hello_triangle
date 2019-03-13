// external crates
#[macro_use]
extern crate if_chain;
// use gl::types::*;
use gl::types::GLfloat;
use engine::camera;
use engine::shader_program;
// modules
mod context;
mod model_creator;
mod event_handler;
mod game_state;
mod game_builder;
use game_builder::*;
mod game_state_renderer;

fn main() -> Result<(), String> {
  start_game()
}

fn start_game() -> Result<(), String> {
  let vertex_glsl: &str = include_str!("../src/glsl/vertex.glsl");
  let fragment_glsl: &str = include_str!("../src/glsl/fragment.glsl");
  let game_builder = GameBuilder::new()
    .with_shaders(vertex_glsl, fragment_glsl)
    .with_name("Hello Dummy");
  let mut game = game_builder.build();
  let vertices: Vec<GLfloat> = vec![
    // X    Y   Z       R     G     B   A
     0.0,  0.5, 0.0,    1.0, 0.0, 0.0, 1.0,
    -0.5, -0.5, 0.0,    0.0, 1.0, 0.0, 1.0,
     0.5, -0.5, 0.0,    0.0, 0.0, 1.0, 1.0,
     0.0,  0.1, 0.1,    1.0, 0.0, 0.0, 1.0, // a smaller triangle in front of the first
    -0.1, -0.1, 0.1,    0.0, 1.0, 0.0, 1.0,
     0.1, -0.1, 0.1,    0.0, 0.0, 1.0, 1.0
  ];
  game.add_model(vertices);
  game.run()
}
