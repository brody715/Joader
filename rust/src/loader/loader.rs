pub struct DataRequest {
    task: TaskRef,
    id: u32,
    dataset: Box<dyn Dataset>
}