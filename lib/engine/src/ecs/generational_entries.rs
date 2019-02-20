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
    if generational_index.index() >= self.0.len() { return None; }
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn set_get_entry() {
    // arrange
    let mut allocator = GenerationalIndexAllocator::default();
    let generational_index = allocator.allocate();
    let mut entries = GenerationalEntries::<u32>::default();
    let expect = 42;
    entries.set(generational_index, expect);
    // act
    let result_opt = entries.get(generational_index);
    // assert
    assert!(result_opt.is_some());
    assert_eq!(expect, *result_opt.expect("yo"));
  }

  #[test]
  fn set_get_two_entry() {
    // arrange
    let mut allocator = GenerationalIndexAllocator::default();
    let gi_a = allocator.allocate();
    let gi_b = allocator.allocate();
    let mut entries = GenerationalEntries::<u32>::default();
    let expect_a = 42;
    entries.set(gi_a, expect_a);
    let expect_b = 1289;
    entries.set(gi_b, expect_b);
    // act
    let result_opt = entries.get(gi_a);
    let result_opt_b = entries.get(gi_b);
    // assert
    assert!(result_opt.is_some());
    assert_eq!(expect_a, *result_opt.expect("yo"));
    assert!(result_opt_b.is_some());
    assert_eq!(expect_b, *result_opt_b.expect("yo"));
  }

  #[test]
  fn new_index_should_get_none() {
    // arrange
    let mut allocator = GenerationalIndexAllocator::default();
    let generational_index = allocator.allocate();
    let mut entries = GenerationalEntries::<u32>::default();
    let expect = 42;
    entries.set(generational_index, expect);
    let new_gi = allocator.allocate();
    // act
    let result_opt = entries.get(new_gi);
    // assert
    assert_eq!(1, new_gi.index());
    assert_eq!(0, new_gi.generation());
    assert!(result_opt.is_none());
  }

    #[test]
  fn reallocated_index_should_get_none() {
    // arrange
    let mut allocator = GenerationalIndexAllocator::default();
    let generational_index = allocator.allocate();
    let mut entries = GenerationalEntries::<u32>::default();
    let expect = 42;
    entries.set(generational_index, expect);
    allocator.deallocate(generational_index);
    let new_gi = allocator.allocate();
    // act
    let result_opt = entries.get(new_gi);
    // assert
    assert_eq!(0, new_gi.index());
    assert_eq!(1, new_gi.generation());
    assert!(result_opt.is_none());
  }
}