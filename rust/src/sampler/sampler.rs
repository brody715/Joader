use rand::Rng;
use std::collections::HashSet;

#[derive(Clone)]
struct Node<'a> {
    values: Vec<&'a str>,
    values_set: HashSet<&'a str>,
    tasks_set: HashSet<u64>,
    left: Option<Box<Node<'a>>>,
    right: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    fn from_slices(val: &[&'a str], tasks: &[u64]) -> Self {
        let mut values = Vec::<&str>::new();
        let mut tasks_set = HashSet::<u64>::new();
        let mut values_set = HashSet::<&str>::new();
        for v in val {
            values.push(*v);
            values_set.insert(*v);
        }

        for id in tasks {
            tasks_set.insert(*id);
        }
        Node {
            values,
            values_set,
            tasks_set,
            left: None,
            right: None,
        }
    }

    fn from_set(values_set: &HashSet<&'a str>, tasks_set: &HashSet<u64>) -> Self {
        Node {
            values: values_set.iter().map(|s| *s).collect::<Vec<&str>>(),
            left: None,
            right: None,
            values_set: values_set.to_owned(),
            tasks_set: tasks_set.to_owned(),
        }
    }

    fn len(&self) -> usize {
        return self.values.len();
    }

    fn set_children(&mut self, left: Box<Node<'a>>, right: Box<Node<'a>>) {
        self.left = Some(left);
        self.right = Some(right);
    }

    fn min_task_length(&self) -> usize {
        let mut l = self.len();
        if let Some(left) = &self.left {
            l += left.len()
        }
        l
    }

    fn intersect_update(&mut self, mut other: Box<Node<'a>>) -> (Box<Node<'a>>, Box<Node<'a>>) {
        let values_set = self
            .values_set
            .intersection(&other.values_set)
            .map(|x| *x)
            .collect::<HashSet<&str>>();
        let tasks_set = self
            .tasks_set
            .union(&other.tasks_set)
            .map(|x| *x)
            .collect::<HashSet<u64>>();
        for v in &values_set {
            self.values_set.remove(*v);
            other.values_set.remove(*v);
        }
        (Box::new(Node::from_set(&values_set, &tasks_set)), other)
    }
    fn push_values(&mut self) {
        for v in &self.values_set {
            self.left.as_mut().unwrap().values.push(*v);
            self.left.as_mut().unwrap().values_set.insert(*v);
            self.right.as_mut().unwrap().values.push(*v);
            self.right.as_mut().unwrap().values_set.insert(*v);
        }
    }

    fn insert(root: Option<Box<Node<'a>>>, mut other: Box<Node<'a>>) -> Box<Node<'a>> {
        if let None = root {
            return other;
        }
        let mut root = root.unwrap();
        if other.len() < root.min_task_length() {
            let (mut new_root, other) = root.intersect_update(other);
            new_root.set_children(other, root);
            return new_root;
        } else {
            let (mut new_root, other) = root.intersect_update(other);
            if let None = root.left {
                new_root.set_children(root, other);
            } else {
                root.push_values();
                new_root.set_children(root.left.unwrap(), root.right.unwrap());
                new_root.right = Some(Node::insert(new_root.right, other));
            }
            return new_root;
        }
    }

    fn get_task_values(&self, task_id: u64) -> Vec<&str> {
        let mut res = Vec::<&str>::new();
        if self.tasks_set.contains(&task_id) {
            for v in &self.values {
                res.push(*v);
            }
            if let Some(left) = &self.left {
                let left_v = left.get_task_values(task_id);
                for v in left_v {
                    res.push(v);
                }
            }
            if let Some(right) = &self.right {
                let right_v = right.get_task_values(task_id);
                for v in right_v {
                    res.push(v);
                }
            }
        }
        res
    }
}

#[derive(Clone)]
pub struct Sampler<'a> {
    root: Option<Box<Node<'a>>>,
    //(task_id, weights_sum)
    tasks: Vec<(u64, usize)>,
}

impl<'a> Sampler<'a> {
    pub fn new() -> Self {
        Sampler {
            root: None,
            tasks: Vec::<(u64, usize)>::new(),
        }
    }
    pub fn insert(&mut self, task_id: u64, weights: &[i32], keys: &[&'a str]) {
        // Now, we only support weight = 1
        let weights_sum = weights.len();
        let mut index = 0usize;
        let len = self.tasks.len();
        while index < len && self.tasks[index].1 < weights_sum {
            index += 1;
        }
        self.tasks.insert(index, (task_id, weights_sum));
        let other = Box::new(Node::from_slices(keys, &[task_id]));
        self.root = Some(Node::insert(self.root.clone(), other));
    }

    pub fn get_task_values(&self, task_id: u64) -> Vec<&str> {
        if let Some(root) = &self.root {
            return root.get_task_values(task_id);
        }
        Vec::<&str>::new()
    }
    pub fn sample(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::task;

    use super::*;
    #[test]
    fn test() {
        test_insert(100);
    }

    fn test_insert(tasks: i32) {
        let mut sampler = Sampler::new();
        let mut rng = rand::thread_rng();
        let mut vec = Vec::<HashSet<String>>::new();
        for i in 0..tasks {
            let size = rng.gen_range(1000..10000);
            let set = (0..size)
                .into_iter()
                .map(|x| x.to_string())
                .collect::<HashSet<String>>();
            vec.push(set);
        }
        for (idx, set) in vec.iter().enumerate() {
            let keys = set.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
            let weights = set.iter().map(|_| 1).collect::<Vec<i32>>();
            sampler.insert(idx as u64, weights.as_ref(), keys.as_ref())
        }
        
        for id in 0..tasks {
            let values = sampler.get_task_values(id as u64);
            let set = values.iter().map(|x| (*x).to_owned()).collect::<HashSet<String>>();
            assert!(set.eq(&vec[(id) as usize]));
        }
    }
}
