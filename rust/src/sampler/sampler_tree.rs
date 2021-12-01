use super::sampler_node::{Node, NodeRef};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Default)]
pub struct SamplerTree {
    root: Option<NodeRef>,
    // (loader_id, loader size)
    loader_set: Vec<(u64, usize)>,
}

impl SamplerTree {
    pub fn new() -> Self {
        SamplerTree {
            root: None,
            loader_set: Vec::new(),
        }
    }

    fn keep_sort(&mut self) {
        let len = self.loader_set.len();
        for i in 0..len {
            if self.loader_set[i].1 > self.loader_set[len - 1].1 {
                let mid = self.loader_set[i].clone();
                self.loader_set[i] = self.loader_set[len - 1];
                self.loader_set[len - 1] = mid;
                return;
            }
        }
    }

    pub fn insert(&mut self, indices: Vec<u32>, id: u64) {
        self.loader_set.push((id, indices.len()));
        self.keep_sort();
        let mut loader_set = HashSet::new();
        loader_set.insert(id);
        let node = Node::new(indices, loader_set);
        if let Some(root) = &self.root {
            self.root = Some(Node::insert(root.clone(), node));
        } else {
            self.root = Some(node);
        }
    }

    pub fn get_task_values(&self, loader_id: u64) -> Vec<u32> {
        if let Some(root) = &self.root {
            return root.as_ref().borrow().get_loader_values(loader_id);
        }
        Vec::new()
    }

    pub fn sample(&mut self) -> HashMap<u32, HashSet<u64>> {
        let mut loaders = Vec::new();
        for loader in &self.loader_set {
            if loader.1 != 0 {
                loaders.push(loader.clone())
            }
        }
        let mut decisions = HashSet::new();
        let mut res = HashMap::<u32, HashSet<u64>>::new();
        Node::decide(
            self.root.clone().unwrap(),
            &mut loaders,
            &mut decisions,
            vec![],
        );
        for decision in decisions {
            let ret = decision.execute();
            res.insert(ret, decision.get_loaders());
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::{iter::FromIterator, time::Instant};
    #[test]
    fn test_sampler() {
        sample(8);
    }

    fn sample(tasks: u64) {
        let mut sampler = SamplerTree::new();
        let mut rng = rand::thread_rng();
        let mut vec_keys = Vec::<HashSet<u32>>::new();
        for id in 0..tasks {
            let size = rng.gen_range(1000..10000);
            let keys = (0..size).into_iter().collect::<Vec<u32>>();
            vec_keys.push(HashSet::from_iter(keys.iter().cloned()));
            sampler.insert(keys, id)
        }

        let mut map: HashMap<u64, HashSet<u32>> = HashMap::new();
        let mut time;
        loop {
            let now = Instant::now();
            let res = sampler.sample();
            time = now.elapsed().as_micros();
            if res.is_empty() {
                break;
            }
            for (x, tasks) in &res {
                for task in tasks {
                    map.get_mut(task).unwrap().insert(*x);
                }
            }
        }
        println!("time cost in one turn: {}", time);
        for (task, set) in &map {
            let keys = &vec_keys[(*task) as usize];
            assert!(keys.eq(set));
        }
    }
    #[test]
    fn test_insert() {
        sample(8);
    }
    // fn insert(tasks: u32) {
    //     let mut sampler = Sampler::new();
    //     let mut rng = rand::thread_rng();
    //     let mut vec_keys = Vec::<Vec<u32>>::new();

    //     for _i in 0..tasks {
    //         let size = rng.gen_range(5..100);
    //         let keys = (0..size).into_iter().collect();
    //         vec_keys.push(keys);
    //     }

    //     let mut vec_tasks = Vec::new();
    //     for (idx, keys) in vec_keys.iter().enumerate() {
    //         let (s, _) = channel::unbounded();
    //         let task = TaskRef::new(idx as u64, 0, &keys, s);
    //         vec_tasks.push(task.clone());
    //         sampler.insert(task);
    //     }

    //     for task in vec_tasks {
    //         let mut values = sampler.get_task_values(task.clone());
    //         values.sort();
    //         let mut keys = task.keys().clone();
    //         keys.sort();
    //         assert!(values.eq(&keys));
    //     }
    // }
}
