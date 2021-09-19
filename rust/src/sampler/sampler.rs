use crate::dataset::Dataset;
use crate::task::{Task, TaskRef};
use rand::Rng;
use std::{borrow::BorrowMut, collections::HashSet, rc::Rc, sync::Arc};

#[derive(Clone)]
struct Node {
    values: Vec<u32>,
    values_set: HashSet<u32>,
    tasks_set: HashSet<TaskRef>,
    // smaller task at left, larger at right
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
}
//sampling
// impl Node {
//     fn sample(
//         &mut self,
//         mut task_set: &Vec<(u64, u64)>,
//     ) -> (HashMap<u64, Vec<u64>>, HashMap<u64, Vec<u64>>) {
//         let mut rng = rand::thread_rng();
//         let mut weight = self.values.len() as u64;
//         let mut decided_task = Vec::new();
//         for (idx, (task_id, task_weight)) in task_set.iter().enumerate() {
//             if rng.gen::<f64>() > weight as f64/(*task_weight as f64) {
//                 break;
//             }
//             decided_task.push(idx);
//             weight = *task_weight;
//         }

//         let mut compensation
//         if (decided_task == 0) {
//             todo!()
//         } else {
//             compensation
//         }
//         todo!()
//     }
//     fn random_choose(&mut self, task_set: HashSet<u64>) -> HashMap<&'a str, HashSet<u64>> {
//         let res = HashMap::new();
//         let mut rng = rand::thread_rng();
//         let choice_idx = rng.gen_range(0..self.values.len());
//         let choice_item = self.values[choice_idx];
//         self.values.remove(choice_idx);
//         let compensation = self.tasks_set.difference(&task_set).map(|x| *x).collect::<HashSet<u64>>();
//         res.insert(choice_item, compensation);
//         res
//     }
//     fn complent() {}
// }

// insert delete

impl Node {
    fn from_task(task: &Arc<Task>) -> Self {
        let values = task.keys();
        let mut values_set = HashSet::new();
        for v in values {
            values_set.insert(*v);
        }
        Node {
            values: values.clone(),
            values_set,
            tasks_set: HashSet::new(),
            left: None,
            right: None,
        }
    }

    fn len(&self) -> usize {
        return self.values.len();
    }

    fn set_children(&mut self, left: Option<Rc<Node>>, right: Option<Rc<Node>>) {
        self.left = left;
        self.right = right;
    }

    fn min_task_length(&self) -> usize {
        let mut l = self.len();
        if let Some(left) = &self.left {
            l += left.len()
        }
        l
    }

    fn intersect_update(&mut self, other: &mut Node) -> Rc<Node> {
        let values_set = self
            .values_set
            .intersection(&other.values_set)
            .map(|x| *x)
            .collect::<HashSet<u32>>();
        let values = values_set.iter().map(|x| *x).collect::<Vec<_>>();
        let tasks_set = self
            .tasks_set
            .union(&other.tasks_set)
            .map(|x| (*x).clone())
            .collect::<HashSet<_>>();
        for v in &values_set {
            self.values_set.remove(v);
            other.values_set.remove(v);
        }

        Rc::new(Node {
            values,
            values_set: values_set,
            tasks_set,
            left: None,
            right: None,
        })
    }
    fn push_values(self: Rc<Node>) -> (Option<Rc<Node>>, Option<Rc<Node>>) {
        if let (Some(mut left), Some(mut right)) = (self.left.clone(), self.right.clone()) {
            let left = Node::get_mut(&mut left);
            let right = Node::get_mut(&mut right);
            for v in &self.values_set {
                left.append_value(*v);
                right.append_value(*v);
            }
            return (self.left.clone(), self.right.clone());
        }
        (Some(self), None)
    }

    fn append_value(&mut self, value: u32) {
        self.values.push(value);
        self.values_set.insert(value);
    }

    // in this tree, we will sample in single process

    fn get_mut(node: &mut Rc<Node>) -> &mut Node {
        unsafe { Rc::get_mut_unchecked(node) }
    }

    fn insert(tree: Option<Rc<Node>>, mut other: Rc<Node>) -> Rc<Node> {
        if let Some(mut tree) = tree {
            let mut new_root;
            {
                let root = Node::get_mut(&mut tree);
                new_root = root.intersect_update(Node::get_mut(&mut other));
            }
            if other.len() < tree.min_task_length() {
                Node::get_mut(&mut new_root).set_children(Some(other), Some(tree));
            } else {
                let (left_tree, right_tree) = tree.push_values();
                Node::get_mut(&mut new_root)
                    .set_children(left_tree, Some(Node::insert(right_tree, other)));
            }
            return new_root;
        }
        other
    }

    fn get_task_values(&self, task: Arc<Task>) -> Vec<u32> {
        let mut res = Vec::<u32>::new();
        let l = self.tasks_set.len();
        let keys = self.tasks_set.iter().collect::<Vec<_>>();
        if self.tasks_set.contains(task.id()) {
            for v in &self.values {
                res.push(*v);
            }
            if let Some(left) = &self.left {
                let left_v = left.get_task_values(task.clone());
                for v in &left_v {
                    res.push(*v);
                }
            }
            if let Some(right) = &self.right {
                let right_v = right.get_task_values(task.clone());
                for v in &right_v {
                    res.push(*v);
                }
            }
        }
        res
    }
}

#[derive(Clone)]
pub struct Sampler {
    root: Option<Rc<Node>>,
    //(key, data_path/url/keys)
    dataset: Arc<dyn Dataset>,
    // store tasks sorted by its length
    task_table: Vec<Arc<Task>>,
}

impl Sampler {
    pub fn new(dataset: Arc<dyn Dataset>) -> Self {
        Sampler {
            root: None,
            dataset,
            task_table: Vec::new(),
        }
    }
    pub fn insert(&mut self, task: Arc<Task>) {
        let index = self
            .task_table
            .binary_search_by_key(&task.len(), |t| t.len())
            .unwrap_or_else(|x| x);
        self.task_table.insert(index, task.clone());
        let node = Rc::new(Node::from_task(&task));
        self.root = Some(Node::insert(self.root.clone(), node));
    }

    pub fn get_task_values(&self, task: Arc<Task>) -> Vec<u32> {
        if let Some(root) = &self.root {
            return root.get_task_values(task);
        }
        Vec::new()
    }
    pub fn sample(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dataset::{DataItem, FileDataset};
    use crossbeam::channel;
    use std::iter::FromIterator;
    #[test]
    fn test() {
        test_insert(3);
    }

    fn test_insert(tasks: u32) {
        let mut sampler = Sampler::new(create_dataset());
        let mut rng = rand::thread_rng();
        let mut vec_keys = Vec::<Vec<u32>>::new();
        for i in 0..tasks {
            let size = rng.gen_range(5..10);
            let keys = (0..size).into_iter().collect();
            vec_keys.push(keys);
        }

        let mut vec_tasks = Vec::new();
        for (idx, keys) in vec_keys.iter().enumerate() {
            let (s, _) = channel::unbounded();
            let task = Arc::new(Task::new(idx as u64, 0, keys.clone(), s));
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
