// external crates
// use gl::types::*;
use gl::types::GLfloat;
use glutin::{GlContext, GlWindow, EventsLoop};
use cgmath::{ Rad, Deg, Matrix4, Point3, Vector3 };
use engine::camera;
use engine::shader_program;
use shader_program::{ ShaderProgram, ShaderProgramBuilder };
use camera::{CameraBuilder, Camera};
use crate::context::setup_context;
use crate::model_creator::add_model;
use crate::game_state::{ GameStateBuilder, GameState };
use crate::event_handler;

pub struct GameBuilder {
  name: String, width: u32, height: u32
}

impl GameBuilder {
  pub fn new() -> Self {
    GameBuilder { 
      name: "Hello GL Window".to_string(),
      width: 1600,
      height: 900
    }
  }

  pub fn with_name(mut self, name: &str) -> Self {
    self.name = name.to_string();
    self
  }

  pub fn with_resolution(mut self, width: u32, height: u32) -> Self {
    self.width = width;
    self.height = height;
    self
  }

  // todo: pub with_clear_color() and other gl settings

  pub fn build(&self) -> Game {
    let (window, events_loop) = setup_context(&self.name, 1600, 900);
    unsafe {
      gl::ClearColor(0.0, 154.0/255.0, 206.0/255.0, 235.0/255.0);
      gl::Enable(gl::DEPTH_TEST);
    }
    let game_state = build_game_state();
    Game {
      window,
      events_loop,
      game_state
    }
  }
}

pub struct Game {
  pub window: GlWindow, 
  pub events_loop: EventsLoop, 
  pub game_state: GameState
}

impl Game {
  pub fn add_model(&mut self, vertices: Vec<GLfloat>) {
    add_model(&mut self.game_state, vertices);
  }

  pub fn run(self) -> Result<(), String> {
    run_game(self.window, self.events_loop, self.game_state)
  }
}

// ==== obsolete stuff from here

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
