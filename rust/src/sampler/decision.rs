use super::sampler_node::NodeRef;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
pub struct Decision {
    node: NodeRef,
    loader_ids: HashSet<u64>,
}

impl Hash for Decision {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        self.node.as_ref().borrow().get_loader_set().hasher();
    }
}

impl PartialEq for Decision {
    fn eq(&self, other: &Self) -> bool {
        self.node.as_ref().borrow().get_loader_set()
            == other.node.as_ref().borrow().get_loader_set()
    }
}

impl Eq for Decision {}

impl Decision {
    pub fn new(node: NodeRef, loader_ids: HashSet<u64>) -> Self {
        Self { node, loader_ids }
    }

    pub fn execute(&self) -> u32 {
        let mut mut_ref = self.node.as_ref().borrow_mut();
        mut_ref.random_choose(self.loader_ids.clone())
    }

    pub fn get_loaders(&self) -> HashSet<u64> {
        self.loader_ids.clone()
    }
}
