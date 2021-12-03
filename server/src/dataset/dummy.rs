use super::Dataset;
use super::DatasetRef;
use crate::{
    cache::cache::Cache,
    proto::dataset::{CreateDatasetRequest, DataItem},
};
use std::{cmp::min, fmt::Debug, sync::Arc};

#[derive(Clone, Default, Debug)]
struct DummyDataset {
    magic: u8,
    items: Vec<DataItem>,
    name: String,
}

pub fn from_proto(request: CreateDatasetRequest) -> DatasetRef {
    let name = request.name;
    let items = request.items;
    Arc::new(DummyDataset {
        items,
        magic: 7u8,
        name,
    })
}

fn len() -> usize {
    256
}

impl Dataset for DummyDataset {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_indices(&self) -> Vec<u32> {
        let start = 0u32;
        let end = self.items.len() as u32;
        (start..end).collect::<Vec<_>>()
    }

    fn read(&self, cache: &mut Cache, idx: u32, ref_cnt: usize) -> u64 {
        let data_name = self.name.clone() + &idx.to_string();
        if let Some(addr) = cache.contains_data(&data_name) {
            return addr as u64;
        }
        let mut len = len();
        let (mut block, idx) = cache.next_block(None, ref_cnt, &data_name);
        let mut block_slice = block.as_mut_slice();
        let mut write_size = min(len, block_slice.len());
        (0..write_size).fold((), |_, i| block_slice[i] = self.magic);
        let mut remain_block = block.occupy(write_size as usize);
        len -= write_size;
        loop {
            let mut last_block = block;
            // write flow:
            // allocate block -> write -> occupy(size)
            // if size < block, then some space remain
            // if size = block, then return None
            // if size == 0, then finish writing and free current block
            if let Some(_b) = remain_block {
                block = _b;
            } else {
                block = cache.next_block(Some(last_block), 0, &data_name).0;
            }
            block_slice = block.as_mut_slice();
            write_size = min(len, block_slice.len());

            (0..write_size).fold((), |_, i| block_slice[i] = self.magic);
            remain_block = block.occupy(write_size as usize);
            len -= write_size;
            if write_size == 0 {
                cache.free_block(block);
                last_block.finish();
                break;
            }
        }
        idx as u64
    }
}
