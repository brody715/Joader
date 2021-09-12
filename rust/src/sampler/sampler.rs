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
        let values = Vec::<&str>::new();
        let tasks_set = HashSet::<u64>::new();
        let values_set = HashSet::<&str>::new();
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

    fn from_set(values_set: HashSet<&'a str>, tasks_set: HashSet<u64>) -> Self {
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

    fn set_left(&mut self, left: Box<Node<'a>>) {
        self.left = Some(left);
    }

    fn set_right(&mut self, right: Box<Node<'a>>) {
        self.right = Some(right);
    }

    fn min_task_length(&self) -> usize {
        let mut l = self.len();
        if let Some(left) = self.left {
            l += left.len()
        }
        l
    }

    fn intersect_update(&mut self, other: &'a mut Node) -> Box<Node<'a>> {
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
        for v in values_set {
            self.values_set.remove(v);
            other.values_set.remove(v);
        }
        Box::new(Node::from_set(values_set, tasks_set))
    }
    fn union(&mut self, other: &Node) -> Box<Node<'a>> {
        todo!()
    }
    fn diff(&mut self, other: &mut Node) {
        todo!()
    }

    fn insert(root: Box<Node<'a>>, other: Box<Node<'a>>) -> Box<Node<'a>> {
        if other.len() < root.min_task_length() {
            let new_root = root.intersect_update(other.as_mut());
            new_root.set_left(other);
            new_root.set_right(root);
            return new_root;
        } else {
            let new_root = root.intersect_update(other.as_mut());
            if let Some(left) = root.left {
                left.union(root.as_ref());
                new_root.set_left(left)
            }
            if let Some(right) = root.right {
                right.union(root.as_ref());
                new_root.set_right(right)
            }
            new_root.set_right(Node::insert(new_root, other));
            return new_root;
        }
    }
}

#[derive(Clone)]
pub struct Sampler<'a> {
    root: Box<Node<'a>>,
    //(task_id, weights_sum)
    tasks: Vec<(u64, usize)>,
}

impl Sampler<'_> {
    pub fn insert(&mut self, task_id: u64, weights: &[i32], keys: &[&str]) {
        // Now, we only support weight = 1
        let weights_sum = weights.len();
        let mut index = 0usize;
        let len = self.tasks.len();
        while index < len && self.tasks[index].1 < weights_sum {
            index += 1;
        }
        self.tasks.insert(index, (task_id, weights_sum));
        let mut node = Node::from_slices(keys, &[task_id]);
        self.root.insert(node);
    }

    pub fn sample(&mut self) {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {

    }
}