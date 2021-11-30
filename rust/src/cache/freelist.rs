use super::head::HEAD_SIZE;
use std::{
    collections::{HashMap, LinkedList},
    rc::Rc,
};

struct Zone {
    start: u64,
    end: u64,
}

impl Zone {
    fn is_valid(&self) -> bool {
        // Now we can only support zone that space is greater than Head
        self.end - self.start > HEAD_SIZE
    }

    fn start(&self) -> u64 {
        self.start
    }

    fn len(&self) -> u64 {
        self.end - self.start
    }
}

pub struct FreeList {
    // the list store the start and the end
    free_list: LinkedList<Rc<Zone>>,
    start_hash: HashMap<u64, Rc<Zone>>,
    end_hash: HashMap<u64, Rc<Zone>>,
}

impl FreeList {
    pub fn new() -> FreeList {
        FreeList {
            free_list: LinkedList::new(),
            start_hash: HashMap::new(),
            end_hash: HashMap::new(),
        }
    }

    pub fn insert(&mut self, off: u64, len: u64) {
        // Todo(xj): merge the continues space
        let mut start = off;
        let mut end = off + len;

        if let Some((_old_end, v)) = self.end_hash.remove_entry(&start) {
            // |old_start .. old_end|start .. end| => |old_start .. end|
            start = v.start;
        }
        if let Some((_old_start, v)) = self.start_hash.remove_entry(&end) {
            // |start .. end|old_start .. old_end| => |start .. end|
            end = v.end;
        }
        self.free_list.push_back(Rc::new(Zone { start, end }));
        self.start_hash
            .insert(start, self.free_list.back_mut().unwrap().clone());
        self.end_hash
            .insert(end, self.free_list.back_mut().unwrap().clone());
    }

    pub fn get(&mut self) -> Option<(u64, u64)> {
        //Todo(xj): find the biggest block
        // find the block larger than head
        let mut max_zone: Option<&Rc<Zone>> = None;
        for zone in self.free_list.iter() {
            if self.is_valid(zone) {
                if let Some(_zone) = max_zone {
                    if zone.len() <= _zone.len() {
                        continue;
                    }
                }
                max_zone = Some(&zone);
            }
        }

        let mut ret = None;
        if let Some(zone) = max_zone {
            ret = Some((zone.start(), zone.len()));
            self.start_hash.remove_entry(&zone.start);
            self.end_hash.remove_entry(&zone.end);
        }
        self.clear();
        ret
    }

    fn clear(&mut self) {
        let mut new_free_list = LinkedList::new();
        for zone in self.free_list.iter() {
            if self.is_valid(zone) {
                new_free_list.push_back(zone.clone());
            }
        }
        self.free_list = new_free_list;
    }

    fn is_valid(&self, zone: &Zone) -> bool {
        zone.is_valid()
            && self.start_hash.contains_key(&zone.start)
            && self.end_hash.contains_key(&zone.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let len = [10, 20, 30, 40, 50];
        let start = 0;
        let mut end = 0;
        let mut space = Vec::new();
        for l in &len {
            space.push((end, *l));
            end += *l;
        }

        let mut fl = FreeList::new();
        for (len, off) in &space {
            fl.insert(*len, *off);
        }
        assert_eq!(fl.get(), Some((start, end)));
        assert_eq!(fl.get(), None);

        let mut max = (0, 0);
        for (idx, (off, len)) in space.iter().enumerate() {
            if (idx & 1) == 0 {
                fl.insert(*off, *len);
                max = (*off, *len);
            }
        }
        assert_eq!(fl.get(), Some(max));
        fl.insert(max.0, max.1);

        for (idx, (off, len)) in space.iter().enumerate() {
            if (idx & 1) == 1 {
                fl.insert(*off, *len);
            }
        }
        assert_eq!(fl.get(), Some((start, end)));
    }
}
