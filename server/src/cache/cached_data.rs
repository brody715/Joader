use std::collections::HashMap;

#[derive(Debug)]
pub struct CachedData {
    data2head: HashMap<String, usize>,
    head2data: HashMap<usize, String>,
}

impl CachedData {
    pub fn new() -> Self {
        CachedData {
            data2head: HashMap::new(),
            head2data: HashMap::new(),
        }
    }

    pub fn add(&mut self, head: usize, data: &str) {
        log::debug!("Cache data {:?} in {:?}", head, data);
        self.data2head.insert(data.to_string(), head);
        self.head2data.insert(head, data.to_string());
    }

    pub fn remove(&mut self, head: usize) {
        let data = self.head2data.remove(&head).unwrap();
        self.data2head.remove(&data).unwrap();
        log::debug!("Free data {:?} in {:?}", head, data);
    }

    pub fn contains(&self, data: &str) -> Option<usize> {
        log::debug!("Hit data {:?}", data);
        self.data2head.get(data).map(|x| *x)
    }
}
