// std
use std::ffi::{CStr, CString};
use std::mem::size_of;
use std::str;
// external crates
extern crate gl;
use gl::types::*;
extern crate glutin;
use glutin::{GlContext, Event, WindowEvent, VirtualKeyCode, ElementState};
extern crate cgmath;
use cgmath::{ Rad, Deg, Matrix, SquareMatrix, Matrix4, PerspectiveFov, Point3, Vector3 };
// modules
mod context;
use context::{ setup_context, Context };
mod shader_program;
use shader_program::create_shader_program;

static mut RUNNING: bool = true;

fn main() {
  start_game();
}

fn start_game(){
  println!("Setting up context...");
  let mut context = setup_context("Hello, Triangle", 1600, 900);
  print_gl_version();

  println!("Creating shader program...");
  let program = unsafe { create_shader_program(include_str!("glsl/vertex.glsl"), include_str!("glsl/fragment.glsl")) };

  println!("Setting up VBO...");
  let vbo = unsafe { setup_vbo() };

  println!("Setting up VAO...");
  let vao = unsafe { setup_vao(vbo) };

  println!("Initializing uniforms...");
  unsafe { init_uniforms(program); }
  
  println!("Running game...");
  run_game(&mut context, vao);
}

fn print_gl_version(){
  let version = unsafe{
    let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
    String::from_utf8(data).unwrap()  // no semicolon means return
  };
  println!("OpenGL Version {}", version);
}

unsafe fn setup_vbo() -> GLuint {
  // ##  Setup vertex data
  let vertices: Vec<GLfloat> = vec![
    // X    Y   Z       R     G     B   A
     0.0,  0.5, 0.0,    1.0, 0.0, 0.0, 1.0,
    -0.5, -0.5, 0.0,    0.0, 1.0, 0.0, 1.0,
     0.5, -0.5, 0.0,    0.0, 0.0, 1.0, 1.0
  ];
  let mut vbo: GLuint = 0;
  gl::GenBuffers(1, &mut vbo);
  gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
  gl::BufferData(
    gl::ARRAY_BUFFER,                                       // target
    (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,  // size in bytes
    vertices.as_ptr() as *const GLvoid,                     // data
    gl::STATIC_DRAW                                         // usage
  );
  gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  vbo
}

unsafe fn setup_vao(vbo: GLuint) -> GLuint {
  let mut vao: GLuint = 0;
  gl::GenVertexArrays(1, &mut vao); 
  gl::BindVertexArray(vao);
  gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

  // location, f/attribute, f/vertex (stride), offset
  setup_attribute(0, 3, 7, 0);  // position
  setup_attribute(1, 4, 7, 3);  // color

  gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  gl::BindVertexArray(0);
  vao
}

unsafe fn setup_attribute(location: GLuint, floats_per_attribute: GLint, floats_per_vertex: usize, offset: usize){
  gl::EnableVertexAttribArray(location); // this is "layout (location = 0)" in vertex shader
  gl::VertexAttribPointer(
    location,   // location
    floats_per_attribute,          // number per attribute
    gl::FLOAT,  // data type
    gl::FALSE,  // normalized
    (floats_per_vertex * size_of::<GLfloat>()) as GLint,  // stride
    (offset * size_of::<GLfloat>()) as *const gl::types::GLvoid  // offset
  );
}

unsafe fn init_uniforms(program: GLuint) {
  let model_matrix: Matrix4<GLfloat> = Matrix4::from_value(1.0);
  let view_matrix: Matrix4<GLfloat> = Matrix4::look_at(
    Point3::new(0.0, 0.0, -2.0),  // eye
    Point3::new(0.0, 0.0, 0.0),   // target
    Vector3::new(0.0, 1.0, 0.0)   // up direction
  );
  let projection_matrix: Matrix4<GLfloat> = Matrix4::from(PerspectiveFov {
    fovy: Rad::from( Deg(45.0) ),
    aspect: 16.0/9.0,
    near: 0.1,
    far: 100.0
  });

  set_uniform_matrix(program, b"Model\0", model_matrix);
  set_uniform_matrix(program, b"View\0", view_matrix);
  set_uniform_matrix(program, b"Projection\0", projection_matrix);
}

unsafe fn set_uniform_matrix(program: GLuint, name: &[u8], matrix: Matrix4<GLfloat>){
  let location = gl::GetUniformLocation(program, name.as_ptr() as *const _);
  println!("setting uniform at location {}", location);
  gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
}

fn run_game(context: &mut Context, vao: GLuint){
  while unsafe{ RUNNING } {
    // handle events
    context.events_loop.poll_events(|event| {
      match event {
        Event::WindowEvent{ event, .. } => {
          match event {
            WindowEvent::Closed => { unsafe{ RUNNING = false; } },
            _ => { handle_window_event(event); }
          }
        },
        _ => ()
      }
    });
    // draw
    unsafe{
      gl::BindVertexArray(vao);
      gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT);
      gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
    // swap buffers
    context.window.swap_buffers().unwrap();
  }
}

fn handle_window_event(event: WindowEvent){
  match event {
    WindowEvent::KeyboardInput {input, ..} => {
      match input.state{
        ElementState::Pressed => {
          if let Some(keycode) = input.virtual_keycode
          {
            match keycode{
              VirtualKeyCode::Escape => { unsafe{ RUNNING = false; } },
              _ => ()
            }
          }
        },
        ElementState::Released => {

        }
      }
    },
    _ => { /* nothing */ }
  }
}