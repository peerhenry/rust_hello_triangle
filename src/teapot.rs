use std::fs::File;
// external crates
#[macro_use]
extern crate if_chain;
// use gl::types::*;
use gl::types::GLfloat;
use engine::camera;
use engine::shader_program;
use fbx3d::decode_fbx;
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
  let game_builder = GameBuilder::new().with_name("Hello Teapot");
  let mut game = game_builder.build();
  let mut f = File::open("teapot.fbx").unwrap();
  let nodes = decode_fbx(&mut f).unwrap();
  
  println!("nodes len {}", nodes.len());
  for node in nodes {
    println!("================================================");
    println!("node name {}", node.name);
    println!("props.len {}", node.properties.len());
    println!("subnodes.len {}", node.subnodes.len());
    for prop in node.properties {
      println!("prop {:?}", prop);
    }
    for subnode in node.subnodes {
      println!("subnode name {}", subnode.name);
      println!("subnode props.len {}", subnode.properties.len());
      println!("subnode subnodes.len {}", subnode.subnodes.len());
    }
  }
  // let vertices: Vec<GLfloat>;
  // game.add_model(vertices);
  game.run()
}
