// std
use std::str;
// external crates
use glutin::{ GlContext, ContextBuilder, WindowBuilder, GlWindow, EventsLoop};

pub fn setup_context(title: &str, width: u32, height: u32) -> (GlWindow, EventsLoop) {
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
  
  (gl_window, events_loop)
}