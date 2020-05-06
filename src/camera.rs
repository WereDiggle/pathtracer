use rand::thread_rng;

use crate::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time_start: f64,
    time_end: f64,
}

impl Camera {
    pub fn new(
        (origin, target, v_up): (Point3, Point3, Point3),
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        (time_start, time_end): (f64, f64),
    ) -> Self {
        let lens_radius = aperture * 0.5;

        let theta = vfov.to_radians();
        let half_height = (theta * 0.5).tan();
        let half_width = aspect_ratio * half_height;

        let w = (origin - target).unit_vector();
        let u = v_up.cross(w).unit_vector();
        let v = w.cross(u);

        Self {
            origin,
            lower_left_corner: origin
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            w,
            u,
            v,
            lens_radius,
            time_start,
            time_end,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let new_origin = self.origin + offset;
        Ray::new(
            new_origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - new_origin,
            thread_rng().gen_range(self.time_start, self.time_end),
        )
    }
}
