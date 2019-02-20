#[derive(Clone, Copy)]
pub struct GenerationalIndex {
  index: usize,
  generation: u64
}

impl GenerationalIndex {
  #[allow(dead_code)]
  pub fn index(&self) -> usize { self.index }

  #[allow(dead_code)]
  pub fn generation(&self) -> u64 { self.generation }
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
      } else {
        let index = self.entries.len();
        self.entries.push(AllocatorEntry {
          is_live: true,
          generation: 0
        });
        (index, 0)
      }
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