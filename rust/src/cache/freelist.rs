use std::collections::VecDeque;

use super::HEAD_SIZE;

pub struct FreeList {
    free_list: VecDeque<(u64, u64)>,
}

impl FreeList {
    pub fn new() -> FreeList {
        FreeList {
            free_list: VecDeque::new(),
        }
    }

    pub fn insert(&mut self, off: u64, len: u64) {
        // Todo(xj): merge the continues space
        self.free_list.push_back((off, len))
    }

    pub fn get(&mut self) -> Option<(u64, u64)> {
        //Todo(xj): find the biggest block
        // find the block larger than head
        for (idx, (_, len)) in self.free_list.iter().enumerate() {
            if *len > HEAD_SIZE {
                self.free_list.swap(idx, 0);
                return self.free_list.pop_front();
            }
        }
        None
    }
}
