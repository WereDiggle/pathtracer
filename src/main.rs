use std::sync::Arc;

pub use clamp::*;
use image::*;

mod camera;
mod hit;
mod hit_list;
mod ray;
mod sphere;
mod vec3;

pub use hit::*;
pub use hit_list::*;
pub use ray::*;
pub use sphere::*;
pub use vec3::*;

fn ray_color(ray: Ray, world: &Arc<dyn Hittable>) -> Vec3 {
    // We hit something
    if let Some(hit_record) = world.hit(&ray, 0.0, std::f64::INFINITY) {
        return 0.5 * (hit_record.normal + Vec3(1.0, 1.0, 1.0));
    }

    // Off into infinity
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

    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    let origin = Vec3(0.0, 0.0, 0.0);

    let mut world = HitList::new();
    world.add(Arc::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)));
    let world: Arc<dyn Hittable> = Arc::new(world);

    for j in 0..image_height {
        for i in 0..image_width {
            let u = i as f64 / image_width as f64;
            let v = j as f64 / image_height as f64;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let color = ray_color(r, &world);

            image_buffer.put_pixel(i, image_height - 1 - j, color.into());
            let progress = ((j * image_width + i) * 100) / (image_height * image_width - 1);
            progress_bar.reach_percent(progress as i32);
        }
    }

    image_buffer.save("output/test.png").unwrap();
}
