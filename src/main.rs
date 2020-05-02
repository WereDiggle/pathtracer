use image::*;

mod ray;
mod vec3;

use ray::*;
use vec3::*;

fn ray_color(ray: Ray) -> Vec3 {
    let unit_direction: Vec3 = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    let image_width: u32 = 200;
    let image_height: u32 = 100;

    let mut image_buffer = DynamicImage::new_rgb8(image_width, image_height).to_rgb();

    let mut progress_bar = progress::Bar::new();
    progress_bar.set_job_title("Rendering...");

    for row in 0..image_height {
        for col in 0..image_width {
            let color = Vec3(
                col as f64 / image_width as f64,
                row as f64 / image_height as f64,
                0.2,
            );

            let r = (255.999 * color.0).floor() as u8;
            let g = (255.999 * color.1).floor() as u8;
            let b = (255.999 * color.2).floor() as u8;

            image_buffer.put_pixel(col, image_height - 1 - row, Rgb([r, g, b]));
            let progress = ((row * image_width + col) * 100) / (image_height * image_width - 1);
            progress_bar.reach_percent(progress as i32);
        }
    }

    image_buffer.save("output/test.png").unwrap();
}
