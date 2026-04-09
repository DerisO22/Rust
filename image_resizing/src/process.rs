use image::GrayImage;
use std::time::Duration;
use std::time::Instant;

use crate::sequential::resize_sequential;
use crate::parallel::resize_parallel;

// Holds the timing results for one resize operation
pub struct ResizeResult {
    pub image_name: String,
    pub src_w:      u32,
    pub src_h:      u32,
    pub dst_w:      u32,
    pub dst_h:      u32,
    pub seq_time:   Duration,
    pub par_time:   Duration,
    pub speedup:    f64,
    pub pixels:     Vec<u8>, // the resized pixel buffer for display
}

// -----------------------------------------------------------------------------
// Runs sequential and parallel resize on `img`, saves the output to disk,
// and returns a ResizeResult with all metrics.
//
// output_path: where to save the resized PNG, e.g. "out_image1_500x500.png"
// -----------------------------------------------------------------------------
pub fn process_image(
    img:         &GrayImage,
    image_name:  &str,
    dst_w:       u32,
    dst_h:       u32,
    output_path: &str,
) -> ResizeResult {
    let (src_w, src_h) = img.dimensions();

    let t0 = Instant::now();
    let seq_pixels = resize_sequential(img, dst_w, dst_h);
    let seq_time = t0.elapsed();

    let t1 = Instant::now();
    let par_pixels = resize_parallel(img, dst_w, dst_h);
    let par_time = t1.elapsed();

    assert_eq!(seq_pixels, par_pixels, "Sequential and parallel results differ for {image_name}!");

    let out_img = GrayImage::from_raw(dst_w, dst_h, par_pixels.clone())
        .expect("Failed to build output image");
    out_img.save(output_path).expect("Failed to save resized image");

    let speedup = seq_time.as_secs_f64() / par_time.as_secs_f64();

    ResizeResult {
        image_name: image_name.to_string(),
        src_w,
        src_h,
        dst_w,
        dst_h,
        seq_time,
        par_time,
        speedup,
        pixels: par_pixels,
    }
}