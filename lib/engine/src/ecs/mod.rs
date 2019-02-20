pub mod generational_entries;
pub mod generational_index;

#[cfg(test)]
mod tests {
  use super::generational_index::*;
  use super::generational_entries::*;

  #[test]
  fn first_allocated_index() {
    // arrange
    let mut allocator = GenerationalIndexAllocator::default();
    // act
    let generational_index: GenerationalIndex = allocator.allocate();
    // assert
    assert_eq!(generational_index.index(), 0);
    assert_eq!(generational_index.generation(), 0);
  }

  #[test]
  fn second_allocated_index() {
    // arrange
    let mut allocator = GenerationalIndexAllocator::default();
    // act
    let generational_index = allocator.allocate();
    let generational_index_b = allocator.allocate();
    // assert
    assert_eq!(generational_index.index(), 0);
    assert_eq!(generational_index.generation(), 0);
    assert_eq!(generational_index_b.index(), 1);
    assert_eq!(generational_index_b.generation(), 0);
  }

  #[test]
  fn second_allocated_deallocate() {
    // arrange
    let mut allocator = GenerationalIndexAllocator::default();
    // act
    let generational_index = allocator.allocate();
    let result = allocator.deallocate(generational_index);
    // assert
    assert_eq!(result, true);
  }

  #[test]
  fn second_allocated_deallocate_allocate() {
    // arrange
    let mut allocator = GenerationalIndexAllocator::default();
    // act
    let generational_index = allocator.allocate();
    let result = allocator.deallocate(generational_index);
    let generational_index = allocator.allocate();
    // assert
    assert_eq!(generational_index.index(), 0);
    assert_eq!(generational_index.generation(), 1);
  }
}
