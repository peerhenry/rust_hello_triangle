// external crates
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

fn main() -> Result<(), String> {
  start_game()
}

fn start_game() -> Result<(), String> {
  let game_builder = GameBuilder::new().with_name("Hello Teapot");
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
