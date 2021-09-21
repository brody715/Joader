use std::rc::Rc;

#[derive(Clone)]
pub struct DataItem {
    keys: Vec<String>
}

impl DataItem {
    pub fn new(keys: Vec<String>) -> Self {
        DataItem{keys}
    }
}

#[derive(Clone)]
pub struct FileDataset {
    dataset: Rc<Vec<DataItem>>
}

pub trait Dataset {
    fn load(&self, index: usize) -> Vec<usize>;
    fn id(&self) -> u32;
}

impl FileDataset {
    pub fn new(dataset: Vec<DataItem>) -> Self {
        FileDataset{dataset: Rc::from(dataset) }
    }

    
}

impl Dataset for FileDataset {
    fn load(&self, index: usize) -> Vec<usize> {
        todo!()
    }
    fn id(&self) -> u32 {
        todo!()
    }
}