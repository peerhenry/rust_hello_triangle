// std
use std::str;
use std::ffi::{CStr};
// external crates
use glutin::{ GlContext, ContextBuilder, WindowBuilder, GlWindow, EventsLoop};

pub fn setup_context(title: &str, width: u32, height: u32) -> (GlWindow, EventsLoop) {
  println!("Setting up window and events loop...");
  let events_loop = EventsLoop::new();
  let window_builder = WindowBuilder::new()
    .with_title(title)
    .with_dimensions(width, height);
  let context_builder = ContextBuilder::new()
    .with_vsync(true);
  let gl_window = GlWindow::new(window_builder, context_builder, &events_loop).unwrap();

  unsafe {
    gl_window.make_current().unwrap();
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    gl::ClearColor(0.0, 154.0/255.0, 206.0/255.0, 235.0/255.0);
  }
  print_gl_version();
  (gl_window, events_loop)
}

fn print_gl_version() {
  let version = unsafe{
    let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
    String::from_utf8(data).unwrap()  // no semicolon means return
  };
  println!("OpenGL Version {}", version);
}