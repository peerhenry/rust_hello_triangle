# Rust Hello Triangle

This is a simple program in [Rust](https://www.rust-lang.org/en-US/) that compiles a basic shader program and renders a triangle.

![Render output](https://github.com/peerhenry/rust_hello_triangle/blob/master/Capture.PNG)

Run the program with `cargo run`

## Update

This project has been refactored to provide scalability.
Features:
- added an update method that rotates the triangle
- segregated responsibilities in modules
- triangle is an entity of components
- builder pattern for generation of VAO
- builder pattern for generation of Camera
- game state is a struct of arrays - instead of containing an array of structs - in prospect of an ECS inspired by the [RustConf keynote](https://www.youtube.com/watch?v=aKLntZcp27M) by Catherine West.)