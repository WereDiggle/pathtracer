use std::sync::Arc;
use std::time::SystemTime;
use std::time::{Duration, Instant};

use chrono::offset::Utc;
use chrono::DateTime;
use image::*;
use rand::random;
use rand::Rng;

mod aabb;
mod bvh;
mod camera;
mod hit;
mod hit_list;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;
mod worker;

pub use aabb::*;
pub use bvh::*;
pub use camera::*;
pub use hit::*;
pub use hit_list::*;
pub use material::*;
pub use ray::*;
pub use sphere::*;
pub use util::*;
pub use vec3::*;
pub use worker::*;

fn main() {
    let config = Config {
        image_width: 1900,
        image_height: 1080,
        samples_per_pixel: 100,
        max_depth: 50,
    };

    let mut image_buffer = DynamicImage::new_rgb8(config.image_width, config.image_height).to_rgb();

    let mut progress_bar = progress::Bar::new();
    progress_bar.set_job_title("Rendering...");
    let world = random_scene();

    let lookfrom = Vec3(50.0, 20.0, -50.0);
    let lookat = Vec3(0.0, 0.0, 0.0);
    let camera = Arc::new(Camera::new(
        (lookfrom, lookat, Vec3(0.0, 1.0, 0.0)),
        20.0,
        config.image_width as f64 / config.image_height as f64,
        0.1,
        50.0,
        (0.0, 1.0),
    ));

    let worker_pool = WorkerPool::spawn(11, world, camera, config.clone());
    let worker_pool = Arc::new(worker_pool);
    let render_start = Instant::now();

    {
        let worker_pool = worker_pool.clone();
        let config = config.clone();
        std::thread::spawn(move || {
            for v in 0..config.image_height {
                for u in 0..config.image_width {
                    worker_pool.send_job(u, v);
                }
            }
        });
    }

    for prog in 0..(config.image_height * config.image_width) {
        let (x, y, color) = worker_pool.recv_color();
        image_buffer.put_pixel(x, config.image_height - 1 - y, color.into());
        let progress = (prog * 100) / (config.image_height * config.image_width - 1);
        progress_bar.reach_percent(progress as i32);
    }

    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    println!("Render took {} seconds", render_start.elapsed().as_secs());

    image_buffer
        .save(format!(
            "output/{}.png",
            datetime.format("%Y_%m_%d_%H_%M_%S")
        ))
        .unwrap();
}

fn ray_color(ray: Ray, world: &Arc<dyn Hittable + Send + Sync>, depth: u32) -> Vec3 {
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

pub fn random_scene() -> Arc<dyn Hittable + Send + Sync> {
    let mut world = HitList::new();

    world.add(Arc::new(Sphere::new(
        Vec3(0.0, -1000000.0, 0.0),
        1000000.0,
        Arc::new(Lambertian::from_albedo(Vec3(0.8, 0.8, 0.8))),
    )));

    let mut rng = rand::thread_rng();

    let range: i32 = 8;
    for a in -range..=range {
        for b in -1..=range {
            let choose_mat: f64 = rng.gen();
            let sphere_size = 2.0 + (2.0f64).powf((b as f64).abs());
            let center = Vec3(
                a as f64 * sphere_size * (2.2),
                sphere_size,
                sphere_size * (5.0) * b.signum() as f64,
            );
            if choose_mat < 0.4 {
                let albedo = Vec3::random() * Vec3::random();
                world.add(Arc::new(Sphere::new(
                    center,
                    sphere_size,
                    Arc::new(Lambertian::from_albedo(albedo)),
                )));
            } else if choose_mat < 0.85 {
                let albedo = Vec3::random_range(0.5, 1.0);
                let fuzz = rng.gen_range(0.0, 0.5);
                world.add(Arc::new(Sphere::new(
                    center,
                    sphere_size,
                    Arc::new(Metal::new(albedo, fuzz)),
                )));
            } else {
                world.add(Arc::new(Sphere::new(
                    center,
                    sphere_size,
                    Arc::new(Dielectric::new(1.5)),
                )));
                let inner_size: f64 = rng.gen_range(0.1, 0.6);
                world.add(Arc::new(Sphere::new(
                    center,
                    -sphere_size * inner_size,
                    Arc::new(Dielectric::new(1.5)),
                )));
            }
        }
    }

    //world.add(Arc::new(Sphere::new(
    //    Vec3(0.0, 1.0, 0.0),
    //    1.0,
    //    Arc::new(Dielectric::new(1.5)),
    //)));
    //world.add(Arc::new(Sphere::new(
    //    Vec3(-4.0, 1.0, 0.0),
    //    1.0,
    //    Arc::new(Lambertian::from_albedo(Vec3(0.4, 0.2, 0.1))),
    //)));
    //world.add(Arc::new(Sphere::new(
    //    Vec3(4.0, 1.0, 0.0),
    //    1.0,
    //    Arc::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0)),
    //)));

    //let world = BVH::from_hit_list(world, (0.0, 1.0));

    Arc::new(world)
}
