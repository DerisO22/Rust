use image::{GrayImage, Luma};

// -----------------------------------------------------------------------------
// Sequential implementation
// Iterates over every output pixel one at a time, in row-major order.
// -----------------------------------------------------------------------------
pub fn resize_sequential(src: &GrayImage, dst_w: u32, dst_h: u32) -> Vec<u8> {
    let (src_w, src_h) = src.dimensions();
    let mut output = vec![0u8; (dst_w * dst_h) as usize];

    for dst_y in 0..dst_h {
        for dst_x in 0..dst_w {
            // Map output coords → nearest source coords
            let src_x = dst_x * src_w / dst_w;
            let src_y = dst_y * src_h / dst_h;

            let Luma([px]) = *src.get_pixel(src_x, src_y);
            output[(dst_y * dst_w + dst_x) as usize] = px;
        }
    }

    output
}