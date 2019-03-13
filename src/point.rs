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
mod triangle_creator;
use triangle_creator::*;
mod game_state_renderer;

fn main() -> Result<(), String> {
  start_game()
}

fn start_game() -> Result<(), String> {
  let vertex_glsl: &str = include_str!("../src/glsl/point_render/vertex.glsl");
  let fragment_glsl: &str = include_str!("../src/glsl/point_render/fragment.glsl");
  let geometry_glsl: &str = include_str!("../src/glsl/point_render/geometry.glsl");
  let game_builder = GameBuilder::new()
    .with_geometry_shader(geometry_glsl)
    .with_mode(gl::POINTS)
    .with_shaders(vertex_glsl, fragment_glsl);
  let mut game = game_builder.build();
  unsafe { gl::PointSize(20.0); }
  add_triangle(&mut game.game_state);
  game.run()
}
