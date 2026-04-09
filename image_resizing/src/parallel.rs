use image::{GrayImage, Luma};
use rayon::prelude::*;

// -----------------------------------------------------------------------------
// Parallel implementation using Rayon
// Same math — but the outer loop is replaced with a parallel iterator.
// Rayon automatically divides the index range across available CPU cores.
// -----------------------------------------------------------------------------
pub fn resize_parallel(src: &GrayImage, dst_w: u32, dst_h: u32) -> Vec<u8> {
    let (src_w, src_h) = src.dimensions();
    let total = (dst_w * dst_h) as usize;

    // par_iter over flat pixel indices: 0, 1, 2, ..., total-1
    (0..total)
        .into_par_iter()
        .map(|idx| {
            let dst_x = (idx as u32) % dst_w;
            let dst_y = (idx as u32) / dst_w;

            // Identical nearest-neighbor formula as sequential
            let src_x = dst_x * src_w / dst_w;
            let src_y = dst_y * src_h / dst_h;

            let Luma([px]) = *src.get_pixel(src_x, src_y);
            px
        })
        .collect()
}