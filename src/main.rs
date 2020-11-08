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
mod onb;
mod perlin;
mod ray;
mod scenes;
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
pub use onb::*;
pub use perlin::*;
pub use ray::*;
pub use scenes::*;
pub use texture::*;
pub use util::*;
pub use vec3::*;
pub use volume::*;
pub use worker::*;
pub use world::*;

fn main() {
    let quality = 5;
    let config = Config {
        image_width: 100 * quality,
        image_height: 100 * quality,
        samples_per_pixel: 10,
        max_depth: 50,
    };

    let mut image_buffer = DynamicImage::new_rgb8(config.image_width, config.image_height).to_rgb();

    // World generation
    let (world, camera) = cornell_box(&config);

    let worker_pool = WorkerPool::spawn(12, world, camera, config.clone());
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
