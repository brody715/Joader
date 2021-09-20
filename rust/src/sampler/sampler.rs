use crate::dataset::Dataset;
use crate::task::{Task, TaskRef};
use rand::prelude::ThreadRng;
use rand::Rng;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ops::Index;
use std::ptr;
use std::{collections::HashSet, rc::Rc, sync::Arc};

struct Node {
    values: Vec<u32>,
    values_set: HashSet<u32>,
    tasks_set: HashSet<TaskRef>,
    rng: ThreadRng,
    // The left is smaller task, and the right is larger
    left: Option<NodeRef>,
    right: Option<NodeRef>,
}

#[derive(Clone)]
struct NodeRef(Rc<Node>);

impl PartialEq for NodeRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for NodeRef {}

impl Hash for NodeRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(Rc::as_ptr(&self.0), state)
    }
}

impl NodeRef {
    fn get_mut(&mut self) -> &mut Node {
        unsafe { Rc::get_mut_unchecked(&mut self.0) }
    }

    fn set_children(&mut self, left: Option<NodeRef>, right: Option<NodeRef>) {
        self.get_mut().left = left;
        self.get_mut().right = right;
    }

    fn len(&self) -> usize {
        return self.0.values.len();
    }

    fn min_task_length(&self) -> usize {
        let mut l = self.len();
        if let Some(left) = &self.0.left {
            l += left.len()
        }
        l
    }

    fn append_value(&mut self, value: u32) {
        let node = self.get_mut();
        node.values.push(value);
        node.values_set.insert(value);
    }

    fn from_task(task: &TaskRef) -> Self {
        let values = task.keys();
        let mut values_set = HashSet::new();
        for v in values {
            values_set.insert(*v);
        }
        let mut tasks_set = HashSet::new();
        tasks_set.insert(task.clone());
        NodeRef(Rc::new(Node {
            values: values.clone(),
            values_set,
            tasks_set,
            rng: rand::thread_rng(),
            left: None,
            right: None,
        }))
    }

    fn intersect_update(&mut self, other: &mut Node) -> NodeRef {
        let values_set = self
            .0
            .values_set
            .intersection(&other.values_set)
            .map(|x| *x)
            .collect::<HashSet<u32>>();
        let values = values_set.iter().map(|x| *x).collect::<Vec<_>>();
        let tasks_set = self
            .0
            .tasks_set
            .union(&other.tasks_set)
            .map(|x| (*x).clone())
            .collect::<HashSet<_>>();
        for v in &values_set {
            self.get_mut().values_set.remove(v);
            other.values_set.remove(v);
        }

        NodeRef(Rc::new(Node {
            values,
            values_set,
            tasks_set,
            rng: rand::thread_rng(),
            left: None,
            right: None,
        }))
    }

    fn push_values(&self) -> (Option<NodeRef>, Option<NodeRef>) {
        if let (Some(mut left), Some(mut right)) = (self.0.left.clone(), self.0.right.clone()) {
            for v in &self.0.values_set {
                left.append_value(*v);
                right.append_value(*v);
            }
            return (self.0.left.clone(), self.0.right.clone());
        }
        (Some(self.clone()), None)
    }

    fn insert(tree: Option<NodeRef>, mut other: NodeRef) -> NodeRef {
        if let Some(mut tree) = tree {
            let mut new_root;
            {
                new_root = tree.intersect_update(other.get_mut());
            }
            if other.len() < tree.min_task_length() {
                new_root.set_children(Some(other), Some(tree));
            } else {
                let (left_tree, right_tree) = tree.push_values();
                new_root.set_children(left_tree, Some(NodeRef::insert(right_tree, other)));
            }
            return new_root;
        }
        other
    }

    fn get_task_values(&self, task: TaskRef) -> Vec<u32> {
        let mut res = Vec::<u32>::new();
        let l = self.0.tasks_set.len();
        let keys = self.0.tasks_set.iter().collect::<Vec<_>>();
        if self.0.tasks_set.contains(task.id()) {
            for v in &self.0.values {
                res.push(*v);
            }
            if let Some(left) = &self.0.left {
                let left_v = left.get_task_values(task.clone());
                for v in &left_v {
                    res.push(*v);
                }
            }
            if let Some(right) = &self.0.right {
                let right_v = right.get_task_values(task.clone());
                for v in &right_v {
                    res.push(*v);
                }
            }
        }
        res
    }
}

// sampling
impl NodeRef {
    fn decide (
        &mut self,
        tasks: &mut Vec<(TaskRef, usize)>,
        decisions: &mut HashMap<NodeRef, HashSet<TaskRef>>,
    ) {
        let common = self.len();
        decisions.insert(self.clone(), HashSet::new());
        let tasks_cloned = tasks.clone();
        for (idx, task) in tasks_cloned.iter().cloned().enumerate() {
            if self.uniform_rand() > (common as f32) / (task.1 as f32) {
                if idx == 0 {
                    if let Some(left) = &self.0.left {
                        let mut value = HashSet::new();
                        value.insert(task.0);
                        decisions.insert(left.clone(), value);
                    }
                }
                break;
            }
            if let Some(task_set) = decisions.get_mut(&self) {
                task_set.insert(task.0);
            }
            tasks.remove(0);
        }
        if !tasks.is_empty() {
            if let Some(mut right) = self.0.right.clone() {
                right.decide(tasks, decisions)
            }
        }
    }

    fn uniform_rand(&mut self) -> f32 {
        self.get_mut().rng.gen()
    }
    fn random_choose(&mut self, task_set: HashSet<TaskRef>) -> u32 {
        let len = self.0.values.len();
        let mut_ref = self.get_mut();
        let choice_idx = mut_ref.rng.gen_range(0..len);
        let choice_item = mut_ref.values[choice_idx];
        mut_ref.values.remove(choice_idx);
        let mut compensation: HashSet<_> =
            HashSet::from_iter(mut_ref.tasks_set.difference(&task_set).cloned());
        self.complent(&mut compensation, choice_item);
        choice_item
    }

    fn complent(&mut self, comp: &mut HashSet<TaskRef>, item: u32) {
        if comp.is_empty() {
            return;
        }
        if self.0.tasks_set.is_superset(comp) {
            self.append_value(item);
        }
        if let (Some(mut left), Some(mut right)) = (self.0.left.clone(), self.0.right.clone()) {
            left.complent(comp, item);
            right.complent(comp, item);
        }
    }
}

#[derive(Clone)]
pub struct Sampler {
    root: Option<NodeRef>,
    //init + get + len
    dataset: Arc<dyn Dataset>,
    // store tasks sorted by its length
    task_table: Vec<(TaskRef, usize)>,
}

impl Sampler {
    pub fn new(dataset: Arc<dyn Dataset>) -> Self {
        Sampler {
            root: None,
            dataset,
            task_table: Vec::new(),
        }
    }
    pub fn insert(&mut self, task: TaskRef) {
        let index = self
            .task_table
            .binary_search_by_key(&task.len(), |t| t.1)
            .unwrap_or_else(|x| x);
        self.task_table.insert(index, (task.clone(), task.len()));
        let node = NodeRef::from_task(&task);
        self.root = Some(NodeRef::insert(self.root.clone(), node));
    }

    pub fn get_task_values(&self, task: TaskRef) -> Vec<u32> {
        if let Some(root) = &self.root {
            return root.get_task_values(task);
        }
        Vec::new()
    }

    pub fn sample(&mut self) -> HashMap::<u32, HashSet<TaskRef>>{
        let mut decisions = HashMap::new();
        let mut tasks = Vec::new();
        for (idx, (_, len)) in self.task_table.iter().enumerate() {
            if *len != 0 {
                tasks = self.task_table[idx..].to_vec();
                break;
            }
        }
        let mut res = HashMap::<u32, HashSet<TaskRef>>::new();
        self.root
            .clone()
            .expect("can not sampling in None")
            .decide(&mut tasks, &mut decisions);
        for (node, tasks) in decisions {
            let mut node = node.clone();
            let id = node.random_choose(tasks.clone());
            res.insert(id, tasks);
        }
        res
    }

    pub fn dataset(&self) -> Arc<dyn Dataset> {
        self.dataset.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dataset::{DataItem, FileDataset};
    use crossbeam::channel;
    use std::iter::FromIterator;
    #[test]
    fn test_sampler() {
        // insert(10);
        sample(10);
    }

    fn sample(tasks: u32) {
        let mut sampler = Sampler::new(create_dataset());
        let mut rng = rand::thread_rng();
        let mut vec_keys = Vec::<Vec<u32>>::new();
        for _i in 0..tasks {
            let size = rng.gen_range(5..100);
            let keys = (0..size).into_iter().collect();
            vec_keys.push(keys);
        }

        let mut vec_tasks = Vec::new();
        for (idx, keys) in vec_keys.iter().enumerate() {
            let (s, _) = channel::unbounded();
            let task = TaskRef::new(idx as u64, 0, keys.clone(), s);
            vec_tasks.push(task.clone());
            sampler.insert(task);
        }

        let mut map = HashMap::<TaskRef, HashSet<u32>>::new();
        for task in &vec_tasks {
            map.insert(task.clone(), HashSet::new());
        }
        loop {
            let res = sampler.sample();
            if res.is_empty() {
                break;
            }
            for (x, tasks) in &res {
                for task in tasks {
                    map.get_mut(task).unwrap().insert(*x);
                }
            }
        }

        for (task, set) in &map {
            let keys: HashSet<_> = HashSet::from_iter(task.keys().iter().cloned());
            assert!(set.eq(&keys));
        }
    }

    fn insert(tasks: u32) {
        let mut sampler = Sampler::new(create_dataset());
        let mut rng = rand::thread_rng();
        let mut vec_keys = Vec::<Vec<u32>>::new();
        for _i in 0..tasks {
            let size = rng.gen_range(5..100);
            let keys = (0..size).into_iter().collect();
            vec_keys.push(keys);
        }

        let mut vec_tasks = Vec::new();
        for (idx, keys) in vec_keys.iter().enumerate() {
            let (s, _) = channel::unbounded();
            let task = TaskRef::new(idx as u64, 0, keys.clone(), s);
            vec_tasks.push(task.clone());
            sampler.insert(task);
        }

        for task in vec_tasks {
            let values = sampler.get_task_values(task.clone());
            let set1: HashSet<_> = HashSet::from_iter(values);
            let set2: HashSet<_> = HashSet::from_iter(task.keys().clone());
            assert!(set1.eq(&set2));
        }
    }

    fn create_dataset() -> Arc<dyn Dataset + 'static> {
        let mut data_items = Vec::new();
        for i in 0..100 {
            data_items.push(DataItem::new(vec![i.to_string()]));
        }
        Arc::new(FileDataset::new(data_items))
    }
}
