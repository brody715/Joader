use crate::dataset::Dataset;
use crate::task::{self, Task, TaskRef};
use rand::distributions::WeightedIndex;
use rand::prelude::{Distribution, ThreadRng};
use rand::{thread_rng, Rng};
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

    fn remove_value(&mut self, value: u32) {
        let node = self.get_mut();
        for (idx, v) in node.values.iter().enumerate() {
            if *v == value {
                node.values.remove(idx);
                break;
            }
        }
        node.values_set.remove(&value);
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

    fn intersect_update(&mut self, other_ref: &mut NodeRef) -> NodeRef {
        let other = other_ref.get_mut();
        let values_set = self
            .0
            .values_set
            .intersection(&other.values_set)
            .cloned()
            .collect::<HashSet<u32>>();
        let values = values_set.iter().cloned().collect::<Vec<_>>();
        let tasks_set = self
            .0
            .tasks_set
            .union(&other.tasks_set)
            .cloned()
            .collect::<HashSet<_>>();
        for v in &values_set {
            self.remove_value(*v);
            other_ref.remove_value(*v);
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

    fn push_values(self) -> (Option<NodeRef>, Option<NodeRef>) {
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
            if other.len() <= tree.min_task_length() {
                new_root = tree.intersect_update(&mut other);
                new_root.set_children(Some(other), Some(tree));
            } else {
                new_root = tree.intersect_update(&mut other);
                let (left_tree, right_tree) = tree.push_values();
                new_root.set_children(left_tree, Some(NodeRef::insert(right_tree, other)));
            }
            return new_root;
        }
        other
    }

    fn get_task_values(&self, task: TaskRef) -> Vec<u32> {
        let mut res = Vec::<u32>::new();
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

    fn get_task_set(&self, task_set: &mut Vec<(TaskRef, usize)>, prefix: usize) {
        if let (Some(left), Some(right)) = (&self.0.left, &self.0.right) {
            let prefix = prefix + self.len();
            let (task, len) = left.get_task(prefix);
            task_set.push((task, len));
            right.get_task_set(task_set, prefix);
            return;
        }
        task_set.push(self.get_task(prefix));
    }

    fn get_task(&self, prefix: usize) -> (TaskRef, usize) {
        assert!(self.0.tasks_set.len() == 1);
        (
            self.0.tasks_set.iter().next().unwrap().clone(),
            self.len() + prefix,
        )
    }
}

// sampling
impl NodeRef {
    fn decide(
        &mut self,
        tasks: &mut Vec<(TaskRef, usize)>,
        decisions: &mut HashMap<NodeRef, HashSet<TaskRef>>,
        mut node_set: Vec<NodeRef>,
    ) {
        node_set.push(self.clone());
        let task_set: HashSet<_> = HashSet::from_iter(tasks.iter().map(|x| x.0.clone()));
        if !self.0.tasks_set.eq(&task_set) {
            if let Some(mut right) = self.0.right.clone() {
                right.decide(tasks, decisions, node_set)
            }
            return;
        }

        let common = node_set.iter().fold(0, |x, n| x + n.len());
        let mut last_common = common;
        let tasks_cloned = tasks.clone();
        let mut first_decision = false;
        for task in tasks_cloned.iter().cloned() {
            let p = self.uniform_rand();
            if p >= (last_common as f32) / (task.1 as f32) {
                break;
            }
            //choose current node
            self.choose_intersection(decisions, &task, &node_set);
            last_common = task.1;
            first_decision = true;
            tasks.remove(0);
        }

        if !tasks.is_empty() {
            for task in tasks.iter_mut() {
                task.1 -= common;
            }
            // choose left child for the first task
            if !first_decision {
                let mut value = HashSet::new();
                value.insert(tasks[0].0.clone());
                decisions.insert(self.0.left.clone().unwrap(), value);
                tasks.remove(0);
            }
            // choose right child
            if let Some(mut right) = self.0.right.clone() {
                right.decide(tasks, decisions, vec![])
            }
        }
    }

    fn choose_intersection(
        &mut self,
        decisions: &mut HashMap<NodeRef, HashSet<TaskRef>>,
        task: &(TaskRef, usize),
        node_set: &Vec<NodeRef>,
    ) {
        let intersection;
        if node_set.len() != 0 {
            let weights = node_set.iter().map(|x| x.len()).collect::<Vec<_>>();
            if weights.iter().sum::<usize>() == 0 {
                return;
            }
            let dist = WeightedIndex::new(&weights).unwrap();
            intersection = &node_set[dist.sample(&mut self.get_mut().rng)];
        } else {
            intersection = &node_set[0];
        }

        if let Some(task_set) = decisions.get_mut(intersection) {
            task_set.insert(task.0.clone());
        } else {
            let mut task_set = HashSet::new();
            task_set.insert(task.0.clone());
            decisions.insert(intersection.clone(), task_set.clone());
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
        mut_ref.values_set.remove(&choice_item);
        let mut compensation: HashSet<_> =
            HashSet::from_iter(mut_ref.tasks_set.difference(&task_set).cloned());
        self.complent(&mut compensation, choice_item);
        choice_item
    }

    fn complent(&mut self, comp: &mut HashSet<TaskRef>, item: u32) {
        if comp.is_empty() {
            return;
        }
        if self.0.tasks_set.is_subset(comp) {
            self.append_value(item);
            for task in &self.0.tasks_set {
                comp.remove(task);
            }
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
    // store tasks sorted by its length
    task_table: Vec<(TaskRef, usize)>,
}

impl Sampler {
    pub fn new() -> Self {
        Sampler {
            root: None,
            task_table: Vec::new(),
        }
    }
    pub fn insert(&mut self, task: TaskRef) {
        let node = NodeRef::from_task(&task);
        self.root = Some(NodeRef::insert(self.root.clone(), node));
        self.task_table.clear();
        self.root
            .as_ref()
            .unwrap()
            .get_task_set(&mut self.task_table, 0);
    }

    pub fn get_task_values(&self, task: TaskRef) -> Vec<u32> {
        if let Some(root) = &self.root {
            return root.get_task_values(task);
        }
        Vec::new()
    }

    pub fn sample(&mut self) -> HashMap<u32, HashSet<TaskRef>> {
        let mut decisions = HashMap::new();
        let mut tasks = self
            .task_table
            .iter()
            .filter(|x| x.1 != 0)
            .cloned()
            .collect::<Vec<_>>();
        let mut res = HashMap::<u32, HashSet<TaskRef>>::new();
        self.root.clone().expect("can not sampling in None").decide(
            &mut tasks,
            &mut decisions,
            vec![],
        );

        for (node, tasks) in decisions {
            let mut node = node.clone();
            let id = node.random_choose(tasks.clone());
            if let Some(v_set) = res.get_mut(&id) {
                for task in tasks {
                    v_set.insert(task);
                }
            } else {
                res.insert(id, tasks);
            }
        }
        for task in &mut self.task_table {
            if task.1 != 0 {
                task.1 -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam::channel;
    use std::time::Instant;
    #[test]
    fn test_sampler() {
        // insert(4);
        sample(8);
    }

    fn sample(tasks: u32) {
        let mut sampler = Sampler::new();
        let mut rng = rand::thread_rng();
        let mut vec_keys = Vec::<Vec<u32>>::new();
        for _i in 0..tasks {
            let size = rng.gen_range(1000..10000);
            let keys = (0..size).into_iter().collect();
            vec_keys.push(keys);
        }

        let mut vec_tasks = Vec::new();
        for (idx, keys) in vec_keys.iter().enumerate() {
            let (s, _) = channel::unbounded();
            let task = TaskRef::new(idx as u64, 0, &keys, s);
            vec_tasks.push(task.clone());
            sampler.insert(task);
        }

        let mut map = HashMap::<TaskRef, Vec<u32>>::new();
        for task in &vec_tasks {
            map.insert(task.clone(), Vec::new());
        }
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
                    map.get_mut(task).unwrap().push(*x);
                }
            }
        }
        println!("time cost in one turn: {}", time);
        for (task, set) in &mut map {
            set.sort();
            let mut keys = task.keys().clone();
            keys.sort();
            assert!(keys.eq(set));
        }
    }

    fn insert(tasks: u32) {
        let mut sampler = Sampler::new();
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
            let task = TaskRef::new(idx as u64, 0, &keys, s);
            vec_tasks.push(task.clone());
            sampler.insert(task);
        }

        for task in vec_tasks {
            let mut values = sampler.get_task_values(task.clone());
            values.sort();
            let mut keys = task.keys().clone();
            keys.sort();
            assert!(values.eq(&keys));
        }
    }
}
