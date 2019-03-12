// external crates
// use gl::types::*;
use gl::types::GLfloat;
use glutin::{GlContext, GlWindow, EventsLoop};
use cgmath::{ Rad, Deg, Matrix4, Point3, Vector3 };
use engine::camera;
use engine::shader_program;
use shader_program::{ ShaderProgram, ShaderProgramBuilder };
use camera::{CameraBuilder, Camera};
// modules
mod context;
use crate::context::setup_context;
mod model_creator;
use crate::model_creator::add_model;
mod event_handler;
mod game_state;
use crate::game_state::{ GameStateBuilder, GameState };

fn main() -> Result<(), String> {
  start_game()
}

fn start_game() -> Result<(), String> {
  let (window, events_loop) = setup_context("Hello, Triangle", 1600, 900);
  unsafe {
    gl::ClearColor(0.0, 154.0/255.0, 206.0/255.0, 235.0/255.0);
    gl::Enable(gl::DEPTH_TEST);
  }
  let mut game_state = build_game_state();
  init_game(&mut game_state);
  run_game(window, events_loop, game_state)
}

fn build_game_state() -> GameState {
  let vertex_glsl: &str = include_str!("../src/glsl/vertex.glsl");
  let fragment_glsl: &str = include_str!("../src/glsl/fragment.glsl");
  let some_program = Some(build_shader_program(vertex_glsl, fragment_glsl));
  if let Some(program) = &some_program { unsafe{ program.get_active_attributes(); } }
  let some_cam = Some(build_camera());
  GameStateBuilder::new()
    .with_shader_program(some_program)
    .with_camera(some_cam)
    .build()
}

fn init_game(game_state: &mut GameState) {
  let vertices: Vec<GLfloat> = vec![
    // X    Y   Z       R     G     B   A
     0.0,  0.5, 0.0,    1.0, 0.0, 0.0, 1.0,
    -0.5, -0.5, 0.0,    0.0, 1.0, 0.0, 1.0,
     0.5, -0.5, 0.0,    0.0, 0.0, 1.0, 1.0,
     0.0,  0.1, 0.1,    1.0, 0.0, 0.0, 1.0, // a smaller triangle in front of the first
    -0.1, -0.1, 0.1,    0.0, 1.0, 0.0, 1.0,
     0.1, -0.1, 0.1,    0.0, 0.0, 1.0, 1.0
  ];
  add_model(game_state, vertices);
}

fn build_shader_program(vertex_glsl: &str, fragment_glsl: &str) -> ShaderProgram {
  ShaderProgramBuilder::new()
    .with_vertex_shader(vertex_glsl)
    .with_fragment_shader(fragment_glsl)
    .build()
}

fn build_camera() -> Camera {
  return CameraBuilder::new()
    .with_eye(Point3::new(0.0, 0.0, -2.0))
    .with_target(Point3::new(0.0, 0.0, 0.0))
    .with_up(Vector3::new(0.0, 1.0, 0.0))
    .with_fovy(Rad::from( Deg(45.0) ))
    .with_aspect(16.0/9.0)
    .with_near(0.1)
    .with_far(100.0)
    .build();
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
  println!("game loop done");
  Ok(())
}

fn update(game: &mut GameState) -> Option<()> {
  for entity_index in &game.entities {
    let model_matrix = game.model_matrices.get(*entity_index)?;
    let rot = Matrix4::from_angle_y(Rad(0.1));
    game.model_matrices.set(*entity_index, rot*model_matrix);
  }
  Some(())
}
