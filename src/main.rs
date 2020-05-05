use std::sync::Arc;

use image::*;
use rand::random;

mod camera;
mod hit;
mod hit_list;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;

pub use camera::*;
pub use hit::*;
pub use hit_list::*;
pub use material::*;
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
        if let Some((attenuation, scatter_ray)) = hit_record.material.scatter(&ray, &hit_record) {
            return attenuation * ray_color(scatter_ray, world, depth - 1);
        }
        return Vec3(0.0, 0.0, 0.0);
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

    let red_diffuse = Arc::new(Lambertian::from_albedo(Vec3(0.7, 0.3, 0.3)));
    let brown_diffuse = Arc::new(Lambertian::from_albedo(Vec3(0.8, 0.8, 0.0)));
    let metal1 = Arc::new(Metal::new(Vec3(0.8, 0.6, 0.2), 1.0));
    let metal2 = Arc::new(Metal::new(Vec3(0.8, 0.8, 0.8), 0.3));
    let glass = Arc::new(Dielectric::new(1.5));

    // Floor
    world.add(Arc::new(Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        brown_diffuse.clone(),
    )));

    // Center
    world.add(Arc::new(Sphere::new(
        Vec3(0.0, 0.0, -1.0),
        0.5,
        red_diffuse.clone(),
    )));

    // Right
    world.add(Arc::new(Sphere::new(
        Vec3(1.0, 0.0, -1.0),
        0.5,
        metal1.clone(),
    )));

    // Left
    world.add(Arc::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        0.5,
        glass.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        -0.45,
        glass.clone(),
    )));

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
