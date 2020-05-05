use crate::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            lower_left_corner: Vec3(-2.0, -1.0, -1.0),
            horizontal: Vec3(4.0, 0.0, 0.0),
            vertical: Vec3(0.0, 2.0, 0.0),
            origin: Point3::zero(),
        }
    }
}

impl Camera {
    pub fn new(
        (origin, target, v_up): (Point3, Point3, Point3),
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let half_height = (theta * 0.5).tan();
        let half_width = aspect_ratio * half_height;

        let w = (origin - target).unit_vector();
        let u = v_up.cross(w).unit_vector();
        let v = w.cross(u);

        Self {
            origin,
            lower_left_corner: origin - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
