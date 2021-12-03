use super::sampler_node::{Node, NodeRef};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Default, Debug)]
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

    pub fn insert(&mut self, indices: Vec<u32>, id: u64) {
        let mut loader_set = HashSet::new();
        loader_set.insert(id);
        let node = Node::new(indices, loader_set);
        if let Some(root) = &mut self.root {
            self.root = Some(root.insert(node));
        } else {
            self.root = Some(node);
        }
        self.loader_set.clear();
        self.root
            .clone()
            .unwrap()
            .get_loader_set(&mut self.loader_set, 0);
    }

    pub fn get_task_values(&self, loader_id: u64) -> Vec<u32> {
        if let Some(root) = &self.root {
            return root.get_loader_values(loader_id);
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
        log::info!("Sampler sample {:?}", loaders);
        let mut decisions = Vec::new();
        let mut res = HashMap::<u32, HashSet<u64>>::new();
        self.root
            .clone()
            .unwrap()
            .decide(&mut loaders, &mut decisions, vec![]);

        for decision in decisions.iter_mut() {
            let ret = decision.execute();
            if let Some(loader_set) = res.get_mut(&ret) {
                for loader in decision.get_loaders() {
                    loader_set.insert(loader);
                }
            } else {
                res.insert(ret, decision.get_loaders());
            }
        }
        for decision in decisions.iter_mut() {
            decision.complent();
        }
        for (_, len) in self.loader_set.iter_mut() {
            if *len != 0 {
                *len -= 1;
            }
        }
        log::info!("Sampler get {:?}", res);
        res
    }

    pub fn is_empty(&self) -> bool {
        let mut loaders = Vec::new();
        for loader in &self.loader_set {
            if loader.1 != 0 {
                loaders.push(loader.clone())
            }
        }
        loaders.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::{iter::FromIterator, time::Instant};
    #[test]
    fn test_sampler() {
        // log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        sample(4);
    }

    fn sample(tasks: u64) {
        let mut sampler = SamplerTree::new();
        let mut rng = rand::thread_rng();
        let mut vec_keys = Vec::<HashSet<u32>>::new();
        let mut map: HashMap<u64, HashSet<u32>> = HashMap::new();

        // let sizes = [1, 2, 4, 8, 16];
        for id in 0..tasks {
            let size = rng.gen_range(1..200);
            // let size = sizes[id as usize];
            let keys = (0..size).into_iter().collect::<Vec<u32>>();
            vec_keys.push(HashSet::from_iter(keys.iter().cloned()));
            sampler.insert(keys, id);
            map.insert(id, HashSet::new());
        }

        let mut time;
        loop {
            let now = Instant::now();
            let res = sampler.sample();
            time = now.elapsed().as_secs_f32();
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
            assert_eq!(keys, set);
        }
    }
    #[test]
    fn test_insert() {
        insert(128);
    }
    fn insert(tasks: u32) {
        let mut sampler = SamplerTree::new();
        let mut rng = rand::thread_rng();
        let mut vec_keys = Vec::<Vec<u32>>::new();

        for _i in 0..tasks {
            let size = rng.gen_range(100..10000);
            let keys = (0..size).into_iter().collect();
            vec_keys.push(keys);
        }

        let vec_tasks = Vec::new();
        for (idx, keys) in vec_keys.iter().enumerate() {
            sampler.insert(keys.clone(), idx as u64);
        }

        for task in vec_tasks {
            let mut values = sampler.get_task_values(task);
            values.sort();
            let mut keys = vec_keys[task as usize].clone();
            keys.sort();
            assert!(values.eq(&keys));
        }
    }
}
