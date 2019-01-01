use Game;
use glutin::{EventsLoop, Event, WindowEvent, VirtualKeyCode, ElementState};

pub fn handle_events_loop(mut events_loop: EventsLoop, game: &mut Game) -> EventsLoop {
  events_loop.poll_events(|event| {
    match event {
      Event::WindowEvent{ event, .. } => { handle_window_event(event, game); },
      _ => ()
    }
  });
  events_loop
}

fn handle_window_event(event: WindowEvent, game: &mut Game) {
  match event {
    WindowEvent::Closed => { game.running = false },
    WindowEvent::KeyboardInput {input, ..} => { handle_key_input(input, game); },
    WindowEvent::MouseInput {state, button, ..} => { handle_mouse_input(state, button, game); },
    WindowEvent::MouseWheel {delta, ..} => { handle_mouse_wheel(delta, game); },
    _ => { }
  }
}

fn handle_key_input(input: glutin::KeyboardInput, game: &mut Game) {
  match input.state {
    ElementState::Pressed => {
      if let Some(keycode) = input.virtual_keycode
      {
        match keycode{
          VirtualKeyCode::Escape => { game.running = false },
          _ => ()
        }
      }
    },
    ElementState::Released => {

    }
  }
}

fn handle_mouse_input(state: glutin::ElementState, button: glutin::MouseButton, game: &mut Game) {

}

fn handle_mouse_wheel(delta: glutin::MouseScrollDelta, game: &mut Game) {

}