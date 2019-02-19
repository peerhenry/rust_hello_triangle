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

fn main() {
  start_game();
}

fn start_game() {
  let (window, events_loop) = setup_context("Hello, Triangle", 1600, 900);
  let program: ShaderProgram = ShaderProgramBuilder::new()
    .with_vertex_shader(include_str!("glsl/vertex.glsl"))
    .with_fragment_shader(include_str!("glsl/fragment.glsl"))
    .build();
  let mut game_state = GameState::new(Some(program));
  init_game(&mut game_state);
  run_game(window, events_loop, game_state);
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

fn run_game(window: GlWindow, events_loop: EventsLoop, mut game_state: GameState) {
  let mut next_loop = events_loop;
  // ggez might have a useful timer, as well as other functionalities like sound
  // https://docs.rs/ggez/0.4.0/ggez/index.html
  loop {
    next_loop = event_handler::handle_events_loop(next_loop, &mut game_state);
    update(&mut game_state);
    draw(&game_state);
    window.swap_buffers().unwrap();
    if !game_state.running {
      break;
    }
  }
}

fn update(game: &mut GameState) {
  for entity_index in &game.entities {
    let model_matrix = game.model_matrices[*entity_index];
    let rot = Matrix4::from_angle_y(Rad(0.1));
    game.model_matrices[*entity_index] = rot * model_matrix;
  }
}

fn draw(game: &GameState) -> Result<(),&str> {
  unsafe { gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT); }
  let program = game.shader_program.as_ref().map_or(Err("Trying to draw but no shader program in GameState"), |p| Ok(p))?;
  let cam = game.camera.as_ref().map_or(Err("Trying to draw but no camera in GameState"), |c| Ok(c))?;
  unsafe {
    program.set_uniform_matrix("View", cam.view_matrix);
    program.set_uniform_matrix("Projection", cam.projection_matrix);
  }
  // todo: move to GameState
  for entity_index in &game.entities {
    let vao = game.vaos[*entity_index];
    let model_matrix = game.model_matrices[*entity_index];
    unsafe {
      program.set_uniform_matrix("Model", model_matrix);
      gl::BindVertexArray(vao);
      gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
  }
  Ok(())
}
