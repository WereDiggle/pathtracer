use std::sync::Arc;

use image::*;
use rand::random;

mod camera;
mod hit;
mod hit_list;
mod ray;
mod sphere;
mod util;
mod vec3;

pub use camera::*;
pub use hit::*;
pub use hit_list::*;
pub use ray::*;
pub use sphere::*;
pub use util::*;
pub use vec3::*;

fn ray_color(ray: Ray, world: &Arc<dyn Hittable>, depth: u32) -> Vec3 {
    // Recursive base case
    if depth == 0 {
        return Vec3::zero();
    }

    // We hit something
    if let Some(hit_record) = world.hit(&ray, 0.001, std::f64::INFINITY) {
        //let target = hit_record.position + hit_record.normal + Vec3::random_unit_vector();
        let target = hit_record.position + Vec3::random_in_hemisphere(hit_record.normal);
        return 0.5
            * ray_color(
                Ray::new(hit_record.position, target - hit_record.position),
                world,
                depth - 1,
            );
    }

    // Off into infinity
    let unit_direction: Vec3 = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    let image_width: u32 = 200;
    let image_height: u32 = 100;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;

    let mut image_buffer = DynamicImage::new_rgb8(image_width, image_height).to_rgb();

    let mut progress_bar = progress::Bar::new();
    progress_bar.set_job_title("Rendering...");

    let mut world = HitList::new();
    world.add(Arc::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)));
    let world: Arc<dyn Hittable> = Arc::new(world);

    let camera = Camera::default();

    for j in 0..image_height {
        for i in 0..image_width {
            let mut total_color = Vec3::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / image_width as f64;
                let v = (j as f64 + random::<f64>()) / image_height as f64;
                let r = camera.get_ray(u, v);
                total_color += ray_color(r, &world, max_depth);
            }
            let final_color = total_color / samples_per_pixel as f64;

            image_buffer.put_pixel(i, image_height - 1 - j, final_color.into());
            let progress = ((j * image_width + i) * 100) / (image_height * image_width - 1);
            progress_bar.reach_percent(progress as i32);
        }
    }

    image_buffer.save("output/test.png").unwrap();
}
