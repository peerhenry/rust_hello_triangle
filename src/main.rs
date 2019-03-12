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
mod triangle_creator;
use triangle_creator::*;

fn main() -> Result<(), String> {
  start_game();
  Ok(())
}

fn start_game() {
  let game_builder = GameBuilder::new();
  let mut game = game_builder.build();
  add_triangle(&mut game.game_state);
  game.run();
}
