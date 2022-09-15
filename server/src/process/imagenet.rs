use super::decode_rgb_from_memory;
use image::imageops::FilterType::Triangle;
use opencv::{
    core::{Range, Vector, CV_8UC3},
    imgproc::resize,
    prelude::{Mat, MatTraitConst, MatTraitConstManual},
};
use rand::distributions::{Distribution, Uniform};
use std::slice::from_raw_parts;
use tch::vision::imagenet::load_image_and_resize224_from_memory;

pub fn random_crop(image: &Mat) -> Mat {
    pub fn random_parame(h: i32, w: i32, scale: &[f32], ratio: &[f32]) -> (i32, i32, i32, i32) {
        let area = (h * w) as f32;
        let ratio_range = Uniform::from(ratio[0].ln()..ratio[1].ln());
        let scale_range = Uniform::from(scale[0]..scale[1]);
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let target_area = area * scale_range.sample(&mut rng);
            let aspect_ratio = ratio_range.sample(&mut rng).exp();
            let crop_w = ((target_area * aspect_ratio).sqrt()).round() as i32;
            let crop_h = ((target_area / aspect_ratio).sqrt()).round() as i32;
            if crop_w > 0 && crop_w <= w && crop_h > 0 && crop_h <= h {
                let h_range = Uniform::from(0..h - crop_h + 1);
                let i = h_range.sample(&mut rng);
                let w_range = Uniform::from(0..w - crop_w + 1);
                let j = w_range.sample(&mut rng);
                return (i, j, crop_h, crop_w);
            }
        }

        // center crop
        let in_ratio = w as f32 / h as f32;
        let crop_w;
        let crop_h;
        if in_ratio < ratio[0] {
            crop_w = w;
            crop_h = (w as f32 / ratio[0]).round() as i32;
        } else if in_ratio > ratio[1] {
            crop_h = h;
            crop_w = (h as f32 * ratio[1]).round() as i32;
        } else {
            crop_h = h;
            crop_w = w;
        }
        let i = (h - crop_h) / 2;
        let j = (w - crop_w) / 2;
        (i, j, crop_h, crop_w)
    }
    let h = image.rows();
    let w = image.cols();
    let (i, j, h, w) = random_parame(h, w, &[0.08, 1.0], &[3.0/4.0, 4.0/3.0]);
    // println!("{:} {:} {:} {:}", i, j, h, w);
    let h_range = Range::new(i, i + h).unwrap();
    let w_range = Range::new(j, j + w).unwrap();
    Mat::ranges(image, &Vector::from(vec![h_range, w_range])).unwrap()
}

pub fn decode_resize_224_opencv(data: &[u8]) -> Vec<u8> {
    let mut rgb = decode_rgb_from_memory(data);
    // let mut rgb = unsafe { Mat::new_rows_cols(image.rows(), image.cols(), CV_8UC3).unwrap() };
    // opencv::imgproc::cvt_color(&mut image, &mut rgb, opencv::imgproc::COLOR_BGR2RGB, 0).unwrap();
    let mut cropped_rgb = random_crop(&mut rgb);
    let mut resized_rgb = unsafe { Mat::new_rows_cols(224, 224, CV_8UC3).unwrap() };
    let size = resized_rgb.size().unwrap();
    resize(&mut cropped_rgb, &mut resized_rgb, size, 0.0, 0.0, opencv::imgproc::INTER_LINEAR).unwrap();
    // opencv::imgcodecs::imwrite("test.jpg", &dst_rgb, &Vector::from_slice(&[opencv::imgcodecs::IMWRITE_JPEG_QUALITY ]) ).unwrap();
    let data = resized_rgb.data_bytes().unwrap().to_vec();
    // println!("{:?}", &data[..10]);
    data
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


mod tests {
    use super::*;
    use crate::{dataset::build_dataset, proto::dataset::{DataItem, CreateDatasetRequest}};

    #[test]
    fn test_resize() {
        let len = 4096;
        let location = "/data/wgc/data/lmdb-imagenet/ILSVRC-train.lmdb".to_string();
        let name = "lmdb".to_string();
        let items = (0..len)
            .map(|x| DataItem {
                keys: vec![x.to_string()],
            })
            .collect::<Vec<_>>();
        let proto = CreateDatasetRequest {
            name,
            location,
            r#type: crate::proto::dataset::create_dataset_request::Type::Lmdb as i32,
            items,
            weights: vec![0],
        };
        let dataset = build_dataset(proto, 0);
        dataset.read(0);
    }
}
