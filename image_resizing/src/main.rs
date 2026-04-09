mod sequential;
mod parallel;
mod process;

use image::{GrayImage, ImageReader, Luma};
use minifb::{Key, Window, WindowOptions};

use process::{process_image, ResizeResult};

const GAP: u32 = 10;
const MAX_PANEL: u32 = 700;

// -----------------------------------------------------------------------------
// CONFIGURATION — edit these to change the image and target sizes
// -----------------------------------------------------------------------------

const IMAGE_PATH: &str = "image_5.png";
const IMAGE_NAME: &str = "Image 5";

const TARGET_SIZES: &[(u32, u32)] = &[
    (16, 16),
    (32, 32),
    (64, 64),
    (128, 128),
    (256, 256),
    (512, 512),
    (1024, 1024),
    (2048, 2048),
];

// -----------------------------------------------------------------------------

fn main() {
    let img = if std::path::Path::new(IMAGE_PATH).exists() {
        ImageReader::open(IMAGE_PATH)
            .expect("Could not open image")
            .decode()
            .expect("Could not decode image")
            .into_luma8()
    } else {
        println!("{IMAGE_PATH} not found — using test pattern instead.");
        generate_test_image(200, 200)
    };

    let (src_w, src_h) = img.dimensions();
    println!("Processing {IMAGE_NAME}  ({src_w}x{src_h})...\n");

    let mut results: Vec<ResizeResult> = Vec::new();

    for (dst_w, dst_h) in TARGET_SIZES {
        let output_path = format!(
            "resized_{}_{}x{}.png",
            IMAGE_PATH.trim_end_matches(".png"),
            dst_w,
            dst_h
        );

        let result = process_image(&img, IMAGE_NAME, *dst_w, *dst_h, &output_path);

        println!(
            "  -> {}x{}  seq: {:.4?}  par: {:.4?}  speedup: {:.3}x  saved: {}",
            dst_w, dst_h,
            result.seq_time,
            result.par_time,
            result.speedup,
            output_path
        );

        results.push(result);
    }

    // --- Print summary table ---
    print_summary(&results);

    /*
    Uncomment this to display the resized images
    //
    for result in &results {
        show_side_by_side(&img, result);
    }
    */
    println!("All done.");
}

// -----------------------------------------------------------------------------
// Prints a formatted summary table of all results
// -----------------------------------------------------------------------------
fn print_summary(results: &[ResizeResult]) {
    println!("\n{:-<72}", "");
    println!(
        "{:<10} {:>12} {:>12} {:>12} {:>12} {:>10}",
        "Image", "Source", "Target", "Seq (ms)", "Par (ms)", "Speedup"
    );
    println!("{:-<72}", "");

    for r in results {
        println!(
            "{:<10} {:>12} {:>12} {:>12.4} {:>12.4} {:>9.2}x",
            r.image_name,
            format!("{}x{}", r.src_w, r.src_h),
            format!("{}x{}", r.dst_w, r.dst_h),
            r.seq_time.as_secs_f64() * 1000.0,
            r.par_time.as_secs_f64() * 1000.0,
            r.speedup,
        );
    }

    println!("{:-<72}", "");
}

// -----------------------------------------------------------------------------
// Opens a minifb window showing the source (left) and resized (right) images.
// Blocks until the user presses ESC or closes the window.
// -----------------------------------------------------------------------------
fn show_side_by_side(src_img: &GrayImage, result: &ResizeResult) {
    let (src_w, src_h) = src_img.dimensions();
    let dst_w = result.dst_w;
    let dst_h = result.dst_h;

    let left_disp_w = MAX_PANEL.min(src_w);
    let left_disp_h = MAX_PANEL.min(src_h);
    let right_disp_w = MAX_PANEL.min(dst_w);
    let right_disp_h = MAX_PANEL.min(dst_h);

    let win_w = left_disp_w + GAP + right_disp_w;
    let win_h = left_disp_h.max(right_disp_h);

    let src_raw = src_img.as_raw();
    let par_pixels = &result.pixels;

    let mut display_buf = vec![0x2a2a2au32; (win_w * win_h) as usize];

    for y in 0..win_h {
        for x in 0..win_w {
            let gray: Option<u8> = if x < left_disp_w && y < left_disp_h {
                let sx = (x * src_w / left_disp_w).min(src_w - 1);
                let sy = (y * src_h / left_disp_h).min(src_h - 1);
                Some(src_raw[(sy * src_w + sx) as usize])
            } else if x < left_disp_w + GAP {
                None
            } else {
                let rx = x - left_disp_w - GAP;
                if rx < right_disp_w && y < right_disp_h {
                    let sx = (rx * dst_w / right_disp_w).min(dst_w - 1);
                    let sy = (y * dst_h / right_disp_h).min(dst_h - 1);
                    Some(par_pixels[(sy * dst_w + sx) as usize])
                } else {
                    None
                }
            };

            if let Some(g) = gray {
                let v = g as u32;
                display_buf[(y * win_w + x) as usize] = (v << 16) | (v << 8) | v;
            }
        }
    }

    let title = format!(
        "{}  |  Original {}x{}  ->  Resized {}x{}  (ESC for next)",
        result.image_name, src_w, src_h, dst_w, dst_h
    );

    let mut window = Window::new(
        &title,
        win_w as usize,
        win_h as usize,
        WindowOptions { resize: false, ..WindowOptions::default() },
    )
    .expect("Failed to create window");

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&display_buf, win_w as usize, win_h as usize)
            .unwrap();
    }
}

// -----------------------------------------------------------------------------
// Generates a checkerboard + gradient test image
// -----------------------------------------------------------------------------
fn generate_test_image(width: u32, height: u32) -> GrayImage {
    GrayImage::from_fn(width, height, |x, y| {
        let checker = ((x / 20) + (y / 20)) % 2 == 0;
        let gradient = ((x + y) * 255 / (width + height)) as u8;
        let value = if checker { gradient } else { 255 - gradient };
        Luma([value])
    })
}