use super::generational_index::*;

struct GenerationalEntry<T> {
  value: T,
  generation: u64
}

pub struct GenerationalEntries<T>(Vec<Option<GenerationalEntry<T>>>);

impl<T> GenerationalEntries<T> {
  pub fn set(&mut self, generational_index: GenerationalIndex, value: T) {
    let index = generational_index.index();
    let new_entry = Some(GenerationalEntry {
      value,
      generation: generational_index.generation()
    });
    if index < self.0.len() {
      self.0[index] = new_entry;
    } else {
      self.0.push(new_entry)
    }
  }

  pub fn get(&self, generational_index: GenerationalIndex) -> Option<&T> {
    let entry = self.0[generational_index.index()].as_ref()?;
    if entry.generation != generational_index.generation() { return None; }
    return Some(&entry.value);
  }

  #[allow(dead_code)]
  pub fn get_mut(&mut self, generational_index: GenerationalIndex) -> Option<&mut T> {
    let entry = self.0[generational_index.index()].as_mut()?;
    if entry.generation != generational_index.generation() { return None; }
    return Some(&mut entry.value);
  }
}

impl<T> Default for GenerationalEntries<T> {
  fn default() -> Self {
    GenerationalEntries(Vec::new())
  }
}