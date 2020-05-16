use std::sync::Arc;
use std::thread::JoinHandle;

use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::*;

pub struct WorkerPool {
    // TODO: We can have JoinHandle later return final render stats upon exit
    workers: Vec<JoinHandle<()>>,
    color_rx: Receiver<(u32, u32, Color3)>,
    job_tx: Sender<(u32, u32)>,
}

impl WorkerPool {
    pub fn spawn(num_workers: usize, world: World, camera: Arc<Camera>, config: Config) -> Self {
        let (color_tx, color_rx) = unbounded::<(u32, u32, Color3)>();
        let (job_tx, job_rx) = unbounded::<(u32, u32)>();
        let mut workers: Vec<JoinHandle<()>> = Vec::with_capacity(num_workers);
        for _ in 0..num_workers {
            let handle = Worker::spawn(
                job_rx.clone(),
                color_tx.clone(),
                world.clone(),
                camera.clone(),
                config.clone(),
            );
            workers.push(handle);
        }
        Self {
            workers: vec![],
            color_rx,
            job_tx,
        }
    }

    pub fn send_job(&self, u: u32, v: u32) {
        self.job_tx.send((u, v)).unwrap();
    }
    pub fn recv_color(&self) -> (u32, u32, Color3) {
        self.color_rx.recv().unwrap()
    }
}

pub struct Worker {
    pub job_rx: Receiver<(u32, u32)>,
    pub color_tx: Sender<(u32, u32, Color3)>,
    pub world: World,
    pub camera: Arc<Camera>,
    pub config: Config,
}

impl Worker {
    pub fn spawn(
        job_rx: Receiver<(u32, u32)>,
        color_tx: Sender<(u32, u32, Color3)>,
        world: World,
        camera: Arc<Camera>,
        config: Config,
    ) -> JoinHandle<()> {
        std::thread::spawn(move || {
            let worker = Worker {
                job_rx,
                color_tx,
                world,
                camera,
                config,
            };
            worker.work_until_dead();
        })
    }

    fn work_until_dead(&self) {
        while let Ok((x, y)) = self.job_rx.recv() {
            let mut total_color = Vec3::zero();
            for _ in 0..self.config.samples_per_pixel {
                let u = (x as f64 + random::<f64>()) / self.config.image_width as f64;
                let v = (y as f64 + random::<f64>()) / self.config.image_height as f64;
                let r = self.camera.get_ray(u, v);
                total_color += self.world.ray_color(r, self.config.max_depth);
            }
            let final_color = total_color / self.config.samples_per_pixel as f64;
            self.color_tx.send((x, y, final_color)).unwrap();
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}
