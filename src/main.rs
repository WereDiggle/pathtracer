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
mod perlin;
mod ray;
mod rectangle;
mod sphere;
mod texture;
mod util;
mod vec3;
mod worker;
mod world;

pub use aabb::*;
pub use bvh::*;
pub use camera::*;
pub use hit::*;
pub use hit_list::*;
pub use material::*;
pub use perlin::*;
pub use ray::*;
pub use rectangle::*;
pub use sphere::*;
pub use texture::*;
pub use util::*;
pub use vec3::*;
pub use worker::*;
pub use world::*;

fn main() {
    let quality = 2;
    let config = Config {
        image_width: 192 * quality,
        image_height: 108 * quality,
        samples_per_pixel: 100,
        max_depth: 50,
    };

    let mut image_buffer = DynamicImage::new_rgb8(config.image_width, config.image_height).to_rgb();

    let mut progress_bar = progress::Bar::new();
    progress_bar.set_job_title("Rendering...");

    // World generation
    let (world, camera) = simple_light(&config);

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

type ThreadHittable = dyn Hittable + Send + Sync;

pub fn earth() -> Arc<ThreadHittable> {
    let earth_texture =
        ImageTexture::from_file(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/earthmap.jpg"));
    let earth_surface = Lambertian::from_texture(Arc::new(earth_texture));
    let globe = Sphere::new(Vec3::zero(), 2.0, Arc::new(earth_surface));

    Arc::new(globe)
}

pub fn two_perlin_spheres() -> Arc<ThreadHittable> {
    let mut world = HitList::new();

    let perlin_texture = Arc::new(NoiseTexture::new(10.0));
    world.add(Arc::new(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(perlin_texture.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(perlin_texture.clone())),
    )));

    Arc::new(world)
}

pub fn simple_light(config: &Config) -> (World, Arc<Camera>) {
    let lookfrom = Vec3(30.0, 0.0, 5.0) + Vec3(0.0, 4.0, 0.0);
    let lookat = Vec3(0.0, 0.0, 0.0);
    let camera = Arc::new(Camera::new(
        (lookfrom, lookat, Vec3(0.0, 1.0, 0.0)),
        20.0,
        config.image_width as f64 / config.image_height as f64,
        0.0,
        10.0,
        (0.0, 1.0),
    ));

    let mut world = HitList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Sphere::arc_new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    ));
    world.add(Sphere::arc_new(
        Vec3(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    ));

    let difflight = Arc::new(DiffuseLight::from_texture(Arc::new(SolidColor::new(
        4.0, 4.0, 4.0,
    ))));
    world.add(Sphere::arc_new(Vec3(0.0, 7.0, 0.0), 2.0, difflight.clone()));
    world.add(Arc::new(AxisRectangle::new(
        "Z",
        (3.0, 5.0),
        (1.0, 3.0),
        (-2.0, -2.0),
        difflight.clone(),
    )));

    let world = World::new(Arc::new(world), Arc::new(SolidColor::new(0.0, 0.0, 0.0)));

    (world, camera)
}

pub fn random_scene(config: &Config) -> (World, Arc<Camera>) {
    let lookfrom = Vec3(30.0, 1.0, 20.0);
    let lookat = Vec3(0.0, 1.0, 0.0);
    let camera = Arc::new(Camera::new(
        (lookfrom, lookat, Vec3(0.0, 1.0, 0.0)),
        20.0,
        config.image_width as f64 / config.image_height as f64,
        0.0,
        10.0,
        (0.0, 1.0),
    ));

    let mut world = HitList::new();

    let checkered = Arc::new(CheckerTexture::new(
        5.0,
        Arc::new(SolidColor::new(0.2, 0.3, 0.1)),
        Arc::new(SolidColor::new(0.9, 0.9, 0.9)),
    ));
    let noise = Arc::new(NoiseTexture::new(10.0));
    let light = Arc::new(DiffuseLight::from_texture(Arc::new(SolidColor::new(
        1.0, 1.0, 1.0,
    ))));
    world.add(Arc::new(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(checkered.clone())),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    //world.add(Arc::new(Sphere::new(
    //    Vec3(0.0, 2.0, 6.0),
    //    2.0,
    //    light.clone(),
    //)));
    world.add(Arc::new(Sphere::new(
        Vec3(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::from_color3(Vec3(0.4, 0.2, 0.1))),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0)),
    )));

    //world.add(SkySphere::from_texture(noise.clone()));
    //let world = BVH::from_hit_list(world, (0.0, 1.0));
    let world = World::new(Arc::new(world), noise.clone());

    (world, camera)
}

pub fn two_spheres() -> Arc<dyn Hittable + Send + Sync> {
    let mut world = HitList::new();
    let checkered = CheckerTexture::new(
        10.0,
        Arc::new(SolidColor::new(0.2, 0.3, 0.1)),
        Arc::new(SolidColor::new(0.9, 0.9, 0.9)),
    );
    let checker_matte = Arc::new(Lambertian::from_texture(Arc::new(checkered)));

    world.add(Arc::new(Sphere::new(
        Vec3(0.0, -10.0, 0.0),
        10.0,
        checker_matte.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3(0.0, 10.0, 0.0),
        10.0,
        checker_matte.clone(),
    )));

    Arc::new(world)
}
