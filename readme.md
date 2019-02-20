# Rust Hello Triangle

This is a simple program in [Rust](https://www.rust-lang.org/en-US/) that compiles a basic shader program and renders a triangle.

![Render output](https://github.com/peerhenry/rust_hello_triangle/blob/master/Capture.PNG)

Run the program with `cargo run`

## Update

This project features:
- An update method that rotates the triangle
- Segregated responsibilities in modules
- Builder pattern for VAO, Camera and ShaderProgram and GameState
- Reusable game logic in a separate library crate (under `lib/engine`)
- A basic ECS - inspired by the [RustConf keynote by Catherine West](https://www.youtube.com/watch?v=aKLntZcp27M).)

## Todo

- FPS counter
- Entity allocator
- Make draw behavior a component in `GameState`