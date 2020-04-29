use image::*;

fn main() {
    let image_width: u32 = 200;
    let image_height: u32 = 100;

    let mut image_buffer = DynamicImage::new_rgb8(image_width, image_height).to_rgb();

    let mut progress_bar = progress::Bar::new();
    progress_bar.set_job_title("Rendering...");

    for row in 0..image_height {
        for col in 0..image_width {
            let r: f64 = col as f64 / image_width as f64;
            let g: f64 = row as f64 / image_height as f64;
            let b: f64 = 0.2;

            let r = (255.999 * r).floor() as u8;
            let g = (255.999 * g).floor() as u8;
            let b = (255.999 * b).floor() as u8;

            image_buffer.put_pixel(col, image_height - 1 - row, Rgb([r,g,b]));
            let progress = ((row*image_width + col)*100)/(image_height*image_width-1);
            progress_bar.reach_percent(progress as i32);
        }
    }

    image_buffer.save("output/test.png").unwrap();
}
