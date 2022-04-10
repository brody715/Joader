use super::decode_from_memory;
use image::imageops::FilterType::Triangle;
use opencv::{
    core::CV_8UC3,
    imgproc::{resize, INTER_LINEAR, INTER_NEAREST},
    prelude::{Boxed, Mat, MatTraitConst, MatTraitConstManual},
};
use std::slice::from_raw_parts;
use tch::vision::imagenet::load_image_and_resize224_from_memory;

pub fn decode_resize_224_opencv(data: &[u8]) -> Vec<u8> {
    let mut image = decode_from_memory(data);
    let mut dst = unsafe { Mat::new_rows_cols(224, 224, CV_8UC3).unwrap() };
    let size = dst.size().unwrap();
    resize(&mut image, &mut dst, size, 0.0, 0.0, INTER_LINEAR).unwrap();
    unsafe { from_raw_parts(dst.as_raw_mut() as *mut u8, 224 * 224 * 3).to_vec() }
}

pub fn decode_resize_224_tch(data: &[u8]) -> Vec<u8> {
    let tensor = load_image_and_resize224_from_memory(data).unwrap();
    let data = unsafe { from_raw_parts(tensor.data_ptr() as *mut u8, 224 * 224 * 3).to_vec() };
    data
}

pub fn decode_resize_224_image(data: &[u8]) -> Vec<u8> {
    let image = image::load_from_memory(data).unwrap();
    let image = image.resize(224, 224, Triangle);
    image.as_bytes().to_vec()
}
