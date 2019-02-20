use crate::shader_program::{ ShaderProgram, SetUniform };
use gl::types::*;
use cgmath::{ Matrix4 };
use crate::camera::Camera;

// GenerationalIndex Hocus Pocus

#[derive(Clone, Copy)]
pub struct GenerationalIndex {
  index: usize,
  generation: u64
}

impl GenerationalIndex {
  #[allow(dead_code)]
  pub fn index(&self) -> usize { self.index }
}

// Allocator

struct AllocatorEntry {
  pub is_live: bool,
  generation: u64
}

#[derive(Default)]
pub struct GenerationalIndexAllocator {
  entries: Vec<AllocatorEntry>,
  free: Vec<usize>
}

impl GenerationalIndexAllocator {
  pub fn allocate(&mut self) -> GenerationalIndex {
    let (index, generation): (usize, u64) = {
      if let Some(free_index) = self.free.pop() {
        let index = free_index;
        let generation = self.entries[free_index].generation + 1;
        (index, generation)
      } else { (self.entries.len(), 0) }
    };
    GenerationalIndex { index, generation }
  }

  // Returns true if the index was allocated before and is now deallocated
  #[allow(dead_code)]
  pub fn deallocate(&mut self, generational_index: GenerationalIndex) -> bool {
    let index = generational_index.index;
    let mut entry_opt = self.entries.get(index);
    if let Some(ref mut entry) = entry_opt {
      let was_live = entry.is_live;
      if was_live {
        self.free.push(index);
        self.entries[index] = AllocatorEntry {
          is_live: false,
          generation: entry.generation
        }
      }
      return was_live
    }
    return false;
  }

  #[allow(dead_code)]
  pub fn is_live(&self, generational_index: GenerationalIndex) -> bool {
    return self.entries[generational_index.index].is_live;
  }
}

// entries

struct GenerationalEntry<T> {
  value: T,
  generation: u64
}

pub struct GenerationalEntries<T>(Vec<Option<GenerationalEntry<T>>>);

impl<T> GenerationalEntries<T> {
  pub fn set(&mut self, generational_index: GenerationalIndex, value: T) {
    let index = generational_index.index;
    let new_entry = Some(GenerationalEntry {
      value,
      generation: generational_index.generation
    });
    if index < self.0.len() {
      self.0[index] = new_entry;
    } else {
      self.0.push(new_entry)
    }
  }

  pub fn get(&self, generational_index: GenerationalIndex) -> Option<&T> {
    let entry = self.0[generational_index.index].as_ref()?;
    if entry.generation != generational_index.generation { return None; }
    return Some(&entry.value);
  }

  #[allow(dead_code)]
  pub fn get_mut(&mut self, generational_index: GenerationalIndex) -> Option<&mut T> {
    let entry = self.0[generational_index.index].as_mut()?;
    if entry.generation != generational_index.generation { return None; }
    return Some(&mut entry.value);
  }
}

impl<T> Default for GenerationalEntries<T> {
  fn default() -> Self {
    GenerationalEntries(Vec::new())
  }
}

// GameState

#[derive(Default)]
pub struct GameState {
  pub entity_allocator: GenerationalIndexAllocator,
  // assets
  pub running: bool,
  pub shader_program: Option<ShaderProgram>,
  pub camera: Option<Camera>,
  // components
  pub vaos: GenerationalEntries<GLuint>,
  pub model_matrices: GenerationalEntries<Matrix4<GLfloat>>,
  pub vertex_counts: GenerationalEntries<GLsizei>,
  // entity indices
  pub entities: Vec<GenerationalIndex>
}

impl GameState {
  pub fn new(shader_program: Option<ShaderProgram>) -> GameState {
    GameState {
      running: true,
      shader_program,
      ..Default::default()
    }
  }

  pub fn draw(&self) -> Result<(),&str> {
    unsafe { gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT); }
    let program = self.shader_program.as_ref().map_or(Err("Trying to draw but no shader program in GameState"), |p| Ok(p))?;
    let cam = self.camera.as_ref().map_or(Err("Trying to draw but no camera in GameState"), |c| Ok(c))?;
    unsafe {
      program.set_uniform_matrix("View", cam.view_matrix);
      program.set_uniform_matrix("Projection", cam.projection_matrix);
    }
    for entity_index in &self.entities {
      self.draw_entity(program, *entity_index);
    }
    Ok(())
  }

  fn draw_entity(&self, program: &ShaderProgram, entity_index: GenerationalIndex) -> Option<()> {
    let vao = *self.vaos.get(entity_index)?;
    let model_matrix = *self.model_matrices.get(entity_index)?;
    let vertex_count = *self.vertex_counts.get(entity_index)?;
    unsafe {
      program.set_uniform_matrix("Model", model_matrix);
      gl::BindVertexArray(vao);
      gl::DrawArrays(gl::TRIANGLES, 0, vertex_count);
    }
    Some(())
  }
}

// builder
#[derive(Default)]
pub struct GameStateBuilder
{
  pub shader_program: Option<ShaderProgram>,
  pub camera: Option<Camera>,
}

impl GameStateBuilder {
  #[allow(dead_code)]
  pub fn new() -> Self {
    Default::default()
  }

  #[allow(dead_code)]
  pub fn with_shader_program(mut self, shader_program: Option<ShaderProgram>) -> Self {
    self.shader_program = shader_program;
    self
  }

  #[allow(dead_code)]
  pub fn with_camera(mut self, camera: Option<Camera>) -> Self {
    self.camera = camera;
    self
  }

  #[allow(dead_code)]
  pub fn build(self) -> GameState {
    let mut state = GameState::new(self.shader_program);
    state.camera = self.camera;
    state
  }
}

#[cfg(test)]
mod game_state_tests {
  use super::*;

  #[test]
  fn can_build_empty_game_state() {
    // arrange
    let builder = GameStateBuilder::new();
    // act
    let game = builder.build();
    // assert
    assert!(game.camera.is_none());
    assert!(game.shader_program.is_none());
  }
}