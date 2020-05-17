use std::sync::Arc;
use std::time::SystemTime;
use std::time::{Duration, Instant};

use chrono::offset::Utc;
use chrono::DateTime;
use image::*;
use rand::random;
use rand::Rng;

mod acceleration;
mod camera;
mod geometry;
mod material;
mod perlin;
mod ray;
mod texture;
mod util;
mod vec3;
mod volume;
mod worker;
mod world;

pub use acceleration::*;
pub use camera::*;
pub use geometry::*;
pub use material::*;
pub use perlin::*;
pub use ray::*;
pub use texture::*;
pub use util::*;
pub use vec3::*;
pub use volume::*;
pub use worker::*;
pub use world::*;

fn main() {
    let quality = 2;
    let config = Config {
        image_width: 192 * quality,
        image_height: 192 * quality,
        samples_per_pixel: 4096,
        max_depth: 50,
    };

    let mut image_buffer = DynamicImage::new_rgb8(config.image_width, config.image_height).to_rgb();

    // World generation
    let (world, camera) = cornell_box(&config);

    let worker_pool = WorkerPool::spawn(11, world, camera, config.clone());
    let worker_pool = Arc::new(worker_pool);
    let render_start = Instant::now();

    let ramp = sampling_ramp(config.samples_per_pixel);

    {
        let worker_pool = worker_pool.clone();
        let config = config.clone();
        let ramp = ramp.clone();
        std::thread::spawn(move || {
            for s in ramp.iter() {
                for v in 0..config.image_height {
                    for u in 0..config.image_width {
                        worker_pool.send_job(*s, u, v);
                    }
                }
            }
        });
    }

    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();

    let mut color_weights =
        vec![(0, Vec3::zero()); (config.image_height * config.image_width) as usize];
    for pass in 1..=ramp.len() {
        let pass_start = Instant::now();
        let mut progress_bar = progress::Bar::new();
        progress_bar.set_job_title(&format!("Rendering Pass {}/{}", pass, ramp.len()));
        for prog in 0..(config.image_height * config.image_width) {
            let (new_weight, x, y, new_color) = worker_pool.recv_color();
            let cur_color_weight = &mut color_weights[(y * config.image_width + x) as usize];
            *cur_color_weight = (
                new_weight + cur_color_weight.0,
                new_color + cur_color_weight.1,
            );
            let color = cur_color_weight.1 / cur_color_weight.0 as f64;

            image_buffer.put_pixel(x, config.image_height - 1 - y, color.into());
            let progress = (prog * 100) / (config.image_height * config.image_width - 1);
            progress_bar.reach_percent(progress as i32);
        }
        println!(
            "\nPass {} took {} seconds\n",
            pass,
            pass_start.elapsed().as_secs()
        );
        image_buffer
            .save(format!(
                "output/{}_{}.png",
                datetime.format("%Y_%m_%d_%H_%M_%S"),
                pass
            ))
            .unwrap();
    }

    println!("Render took {} seconds", render_start.elapsed().as_secs());
}

fn sampling_ramp(mut total_samples: u32) -> Vec<u32> {
    let mut sampling_ramp = vec![];
    let mut cur: u32;
    while total_samples > 64 {
        cur = total_samples / 2;
        total_samples -= cur;
        sampling_ramp.push(cur);
    }
    sampling_ramp.push(total_samples);
    sampling_ramp.into_iter().rev().collect()
}

type ThreadHittable = dyn Hittable + Send + Sync;

pub fn earth() -> Arc<ThreadHittable> {
    let earth_texture =
        ImageTexture::from_file(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/earthmap.jpg"));
    let earth_surface = Lambertian::from_texture(earth_texture);
    Sphere::new(Vec3::zero(), 2.0, earth_surface)
}

pub fn two_perlin_spheres() -> Arc<ThreadHittable> {
    let mut world = HitList::new();

    let perlin_texture = NoiseTexture::new(10.0);
    world.add(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::from_texture(perlin_texture.clone()),
    ));
    world.add(Sphere::new(
        Vec3(0.0, 2.0, 0.0),
        2.0,
        Lambertian::from_texture(perlin_texture.clone()),
    ));

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

    let pertext = NoiseTexture::new(4.0);
    world.add(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::from_texture(pertext.clone()),
    ));
    world.add(Sphere::new(
        Vec3(0.0, 2.0, 0.0),
        2.0,
        Lambertian::from_texture(pertext.clone()),
    ));

    let difflight = DiffuseLight::from_texture(SolidColor::new(4.0, 4.0, 4.0));
    world.add(Sphere::new(Vec3(0.0, 7.0, 0.0), 2.0, difflight.clone()));
    world.add(AxisRectangle::new(
        "Z",
        (3.0, 5.0),
        (1.0, 3.0),
        (-2.0, -2.0),
        difflight.clone(),
    ));

    let world = World::new(Arc::new(world), SolidColor::new(0.0, 0.0, 0.0));

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

    let checkered = CheckerTexture::new(
        5.0,
        SolidColor::new(0.2, 0.3, 0.1),
        SolidColor::new(0.9, 0.9, 0.9),
    );
    let noise = NoiseTexture::new(10.0);
    let light = DiffuseLight::from_texture(SolidColor::new(1.0, 1.0, 1.0));
    world.add(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::from_texture(checkered.clone()),
    ));

    world.add(Sphere::new(Vec3(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)));
    //world.add(Arc::new(Sphere::new(
    //    Vec3(0.0, 2.0, 6.0),
    //    2.0,
    //    light.clone(),
    //)));
    world.add(Sphere::new(
        Vec3(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::from_color3(Vec3(0.4, 0.2, 0.1)),
    ));
    world.add(Sphere::new(
        Vec3(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Vec3(0.7, 0.6, 0.5), 0.0),
    ));

    //world.add(SkySphere::from_texture(noise.clone()));
    //let world = BVH::from_hit_list(world, (0.0, 1.0));
    let world = World::new(Arc::new(world), noise.clone());

    (world, camera)
}

pub fn two_spheres() -> Arc<dyn Hittable + Send + Sync> {
    let mut world = HitList::new();
    let checkered = CheckerTexture::new(
        10.0,
        SolidColor::new(0.2, 0.3, 0.1),
        SolidColor::new(0.9, 0.9, 0.9),
    );
    let checker_matte = Lambertian::from_texture(checkered);

    world.add(Sphere::new(
        Vec3(0.0, -10.0, 0.0),
        10.0,
        checker_matte.clone(),
    ));
    world.add(Sphere::new(
        Vec3(0.0, 10.0, 0.0),
        10.0,
        checker_matte.clone(),
    ));

    Arc::new(world)
}

pub fn cornell_box(config: &Config) -> (World, Arc<Camera>) {
    let lookfrom = Vec3(278.0, 278.0, -800.0);
    let lookat = Vec3(278.0, 278.0, 0.0);
    let camera = Arc::new(Camera::new(
        (lookfrom, lookat, Vec3(0.0, 1.0, 0.0)),
        40.0,
        config.image_width as f64 / config.image_height as f64,
        0.0,
        10.0,
        (0.0, 1.0),
    ));

    let mut world = HitList::new();

    let red = Lambertian::from_rgb(0.65, 0.05, 0.05);
    let white = Lambertian::from_rgb(0.73, 0.73, 0.73);
    let green = Lambertian::from_rgb(0.12, 0.45, 0.15);
    let light = DiffuseLight::from_texture(SolidColor::new(15.0, 15.0, 15.0));

    world.add(FlipFace::new(AxisRectangle::new(
        "X",
        (555.0, 555.0),
        (0.0, 555.0),
        (0.0, 555.0),
        green.clone(),
    )));

    world.add(AxisRectangle::new(
        "X",
        (0.0, 0.0),
        (0.0, 555.0),
        (0.0, 555.0),
        red.clone(),
    ));

    world.add(AxisRectangle::new(
        "Y",
        (213.0, 343.0),
        (554.0, 554.0),
        (227.0, 332.0),
        light,
    ));

    world.add(FlipFace::new(AxisRectangle::new(
        "Y",
        (0.0, 555.0),
        (0.0, 0.0),
        (0.0, 555.0),
        white.clone(),
    )));

    world.add(AxisRectangle::new(
        "Y",
        (0.0, 555.0),
        (555.0, 555.0),
        (0.0, 555.0),
        white.clone(),
    ));

    world.add(FlipFace::new(AxisRectangle::new(
        "Z",
        (0.0, 555.0),
        (0.0, 555.0),
        (555.0, 555.0),
        white.clone(),
    )));

    let cube1 = Cube::new(Vec3::zero(), Vec3(165.0, 330.0, 165.0), white.clone());
    let cube1 = YRotation::new(cube1, 15.0);
    let cube1 = Translation::new(cube1, Vec3(265.0, 0.0, 295.0));
    let cube1 = ConstantMedium::new(cube1, SolidColor::new(0.0, 0.0, 0.0), 0.01);
    world.add(cube1);

    let cube2 = Cube::new(Vec3::zero(), Vec3(165.0, 165.0, 165.0), white.clone());
    let cube2 = YRotation::new(cube2, -18.0);
    let cube2 = Translation::new(cube2, Vec3(130.0, 0.0, 65.0));
    let cube2 = ConstantMedium::new(cube2, SolidColor::new(1.0, 1.0, 1.0), 0.01);
    world.add(cube2);

    let world = World::new(Arc::new(world), SolidColor::new(0.0, 0.0, 0.0));
    (world, camera)
}
