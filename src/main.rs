// external crates
use gl::types::*;
use glutin::{GlContext, GlWindow, EventsLoop};
use cgmath::{ Rad, Deg, Matrix, Matrix4, Point3, Vector3 };
// modules
mod context;
use crate::context::setup_context;
mod shader_program;
use shader_program::ShaderProgram;
mod shader_program_builder;
use shader_program_builder::ShaderProgramBuilder;
mod game_state;
use crate::game_state::GameState;
mod camera;
use crate::camera::{CameraBuilder};
mod triangle_creator;
use crate::triangle_creator::add_triangle;
mod vao_builder;
mod event_handler;

fn main() -> Result<(), String> {
  start_game()
}

fn start_game() -> Result<(), String> {
  let (window, events_loop) = setup_context("Hello, Triangle", 1600, 900);
  let program: ShaderProgram = ShaderProgramBuilder::new()
    .with_vertex_shader(include_str!("glsl/vertex.glsl"))
    .with_fragment_shader(include_str!("glsl/fragment.glsl"))
    .build();
  let mut game_state = GameState::new(Some(program));
  init_game(&mut game_state);
  run_game(window, events_loop, game_state)
}

fn init_game(game_state: &mut GameState) {
  add_triangle(game_state);
  init_camera(game_state);
}

fn init_camera(game_state: &mut GameState) {
  game_state.camera = Some(CameraBuilder::new()
    .with_eye(Point3::new(0.0, 0.0, -2.0))
    .with_target(Point3::new(0.0, 0.0, 0.0))
    .with_up(Vector3::new(0.0, 1.0, 0.0))
    .with_fovy(Rad::from( Deg(45.0) ))
    .with_aspect(16.0/9.0)
    .with_near(0.1)
    .with_far(100.0)
    .build());
}

fn run_game(window: GlWindow, events_loop: EventsLoop, mut game_state: GameState) -> Result<(), String> {
  let mut next_loop = events_loop;
  // ggez might have a useful timer, as well as other functionalities like sound
  // https://docs.rs/ggez/0.4.0/ggez/index.html
  loop {
    next_loop = event_handler::handle_events_loop(next_loop, &mut game_state);
    update(&mut game_state);
    game_state.draw()?;
    window.swap_buffers().unwrap();
    if !game_state.running {
      break;
    }
  }
  Ok(())
}

fn update(game: &mut GameState) {
  for entity_index in &game.entities {
    let model_matrix = game.model_matrices[*entity_index];
    let rot = Matrix4::from_angle_y(Rad(0.1));
    game.model_matrices[*entity_index] = rot * model_matrix;
  }
}
