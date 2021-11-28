use crate::proto::dataloader::CreateDataloaderRequest;

// Loader store the information of schema, dataset and filter
#[derive(Default, Debug)]
pub struct Loader {
    dataset: String,
}

// Loader table store the information of dataset_table and loader
#[derive(Default, Debug)]
pub struct LoaderTable {}

impl Loader {
    pub fn from_proto(req: CreateDataloaderRequest) -> Loader {
        Loader {
            dataset: req.dataset_name,
        }
    }
}

impl LoaderTable {
    pub fn new() -> LoaderTable {
        todo!()
    }

    pub fn insert(&mut self, loader: Loader) -> Result<(), String> {
        todo!()
    }
}
