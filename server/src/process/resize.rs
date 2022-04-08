use opencv::imgproc::{resize as cv_resize, INTER_LINEAR};
use opencv::prelude::MatTraitConstManual;
use opencv::{core::CV_8UC3, prelude::Mat};
pub fn resize(src: &mut Mat, rows: i32, cols: i32) -> Mat {
    let mut dst = unsafe { Mat::new_rows_cols(rows, cols, CV_8UC3).unwrap() };
    let size = dst.size().unwrap();
    cv_resize(src, &mut dst, size, 0.0, 0.0, INTER_LINEAR).unwrap();
    dst
}
