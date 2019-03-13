// use gl::types::*;
use gl::types::{GLfloat, GLenum};
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
use crate::game_state_renderer::{ GameStateRenderer };

pub struct GameBuilder {
  name: String,
  width: u32,
  height: u32,
  vertex_glsl: Option<String>,
  fragment_glsl: Option<String>,
  geometry_glsl: Option<String>,
  mode: GLenum
}

impl GameBuilder {
  pub fn new() -> Self {
    GameBuilder { 
      name: "Hello GL Window".to_string(),
      width: 1600,
      height: 900,
      vertex_glsl: None,
      fragment_glsl: None,
      geometry_glsl: None,
      mode: gl::TRIANGLES
    }
  }

  #[allow(dead_code)]
  pub fn with_name(mut self, name: &str) -> Self {
    self.name = name.to_string();
    self
  }

  #[allow(dead_code)]
  pub fn with_resolution(mut self, width: u32, height: u32) -> Self {
    self.width = width;
    self.height = height;
    self
  }

  #[allow(dead_code)]
  pub fn with_shaders(mut self, vertex_glsl: &str, fragment_glsl: &str) -> Self {
    self.vertex_glsl = Some(vertex_glsl.to_string());
    self.fragment_glsl = Some(fragment_glsl.to_string());
    self
  }

  #[allow(dead_code)]
  pub fn with_geometry_shader(mut self, geometry_glsl: &str) -> Self {
    self.geometry_glsl = Some(geometry_glsl.to_string());
    self
  }

  #[allow(dead_code)]
  pub fn with_mode(mut self, mode: GLenum) -> Self {
    self.mode = mode;
    self
  }

  // todo: pub with_clear_color() and other gl settings

  pub fn build(self) -> Game {
    let (window, events_loop) = setup_context(&self.name, 1600, 900);
    unsafe {
      gl::ClearColor(0.0, 154.0/255.0, 206.0/255.0, 235.0/255.0);
      gl::Enable(gl::DEPTH_TEST);
    }
    let renderer = GameStateRenderer::new(self.mode);
    let game_state = build_game_state(self);
    Game {
      window,
      events_loop,
      game_state,
      renderer
    }
  }
}

pub struct Game {
  pub window: GlWindow, 
  pub events_loop: EventsLoop, 
  pub game_state: GameState,
  pub renderer: GameStateRenderer
}

impl Game {
  #[allow(dead_code)]
  pub fn add_model(&mut self, vertices: Vec<GLfloat>) {
    add_model(&mut self.game_state, vertices);
  }

  pub fn run(self) -> Result<(), String> {
    run_game(self)
  }
}

fn build_game_state(game_builder: GameBuilder) -> GameState {
  let some_program = if_chain!{
    if let Some(vertex_glsl) = game_builder.vertex_glsl;
    if let Some(fragment_glsl) = game_builder.fragment_glsl;
    then {
      let mut builder = ShaderProgramBuilder::new()
        .with_vertex_shader(&vertex_glsl)
        .with_fragment_shader(&fragment_glsl);
      if let Some(geometry_glsl) = game_builder.geometry_glsl { 
        builder = builder.with_geometry_shader(&geometry_glsl);
      }
      let program = builder.build();
      Some(program)
    } else {
      None
    }
  };
  if let Some(program) = &some_program { unsafe{ program.get_active_attributes(); } }
  let some_cam = Some(build_camera());
  GameStateBuilder::new()
    .with_shader_program(some_program)
    .with_camera(some_cam)
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

fn run_game(game: Game) -> Result<(), String> {
  let mut next_loop = game.events_loop;
  let window = game.window;
  let mut game_state = game.game_state;
  let renderer = game.renderer;
  // ggez might have a useful timer, as well as other functionalities like sound
  // https://docs.rs/ggez/0.4.0/ggez/index.html
  loop {
    next_loop = event_handler::handle_events_loop(next_loop, &mut game_state);
    update(&mut game_state);
    renderer.draw(&game_state)?;
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

