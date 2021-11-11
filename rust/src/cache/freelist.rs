use std::collections::VecDeque;

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
        self.free_list.pop_front()
    }
}
