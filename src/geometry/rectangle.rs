use crate::*;

impl From<&str> for Axis {
    fn from(axis: &str) -> Self {
        match axis {
            "x" | "X" => Self::X,
            "y" | "Y" => Self::Y,
            "z" | "Z" => Self::Z,
            err => panic!("No such axis: {}", err),
        }
    }
}

pub struct AxisRectangle {
    material: Arc<dyn Material + Send + Sync>,
    hort: (f64, f64),
    vert: (f64, f64),
    face: f64,
    axis: Axis,
}

impl AxisRectangle {
    pub fn new<A: Into<Axis>>(
        axis: A,
        x: (f64, f64),
        y: (f64, f64),
        z: (f64, f64),
        material: Arc<dyn Material + Send + Sync>,
    ) -> Arc<Self> {
        let axis = axis.into();
        let (face, hort, vert) = match axis.into() {
            Axis::X => (x.0, y, z),
            Axis::Y => (y.0, x, z),
            Axis::Z => (z.0, x, y),
        };

        assert!(hort.0 < hort.1);
        assert!(vert.0 < vert.1);

        Arc::new(Self {
            material,
            axis: axis.into(),
            face,
            hort,
            vert,
        })
    }
}

impl Hittable for AxisRectangle {
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(match self.axis {
            Axis::X => AABB::new(
                Vec3(self.face - 0.0001, self.hort.0, self.vert.0),
                Vec3(self.face + 0.0001, self.hort.1, self.vert.1),
            ),
            Axis::Y => AABB::new(
                Vec3(self.hort.0, self.face - 0.0001, self.vert.0),
                Vec3(self.hort.1, self.face + 0.0001, self.vert.1),
            ),
            Axis::Z => AABB::new(
                Vec3(self.hort.0, self.vert.0, self.face - 0.0001),
                Vec3(self.hort.1, self.vert.1, self.face + 0.0001),
            ),
        })
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let distance = match self.axis {
            Axis::X => (self.face - r.origin.x()) / r.direction.x(),
            Axis::Y => (self.face - r.origin.y()) / r.direction.y(),
            Axis::Z => (self.face - r.origin.z()) / r.direction.z(),
        };
        if distance < t_min || distance > t_max {
            return None;
        }

        let hort = match self.axis {
            Axis::X => r.origin.y() + distance * r.direction.y(),
            Axis::Y | Axis::Z => r.origin.x() + distance * r.direction.x(),
        };
        if hort < self.hort.0 || hort > self.hort.1 {
            return None;
        }
        let vert = match self.axis {
            Axis::X | Axis::Y => r.origin.z() + distance * r.direction.z(),
            Axis::Z => r.origin.y() + distance * r.direction.y(),
        };
        if vert < self.vert.0 || vert > self.vert.1 {
            return None;
        }

        let face = match self.axis {
            Axis::X => r.origin.x() + distance * r.direction.x(),
            Axis::Y => r.origin.y() + distance * r.direction.y(),
            Axis::Z => r.origin.z() + distance * r.direction.z(),
        };
        let mut hit = HitRecord::from_material(self.material.clone());
        hit.u = (hort - self.hort.1) / (self.hort.1 - self.hort.0);
        hit.v = (vert - self.vert.1) / (self.vert.1 - self.vert.0);
        hit.position = match self.axis {
            Axis::X => Vec3(face, hort, vert),
            Axis::Y => Vec3(hort, face, vert),
            Axis::Z => Vec3(hort, vert, face),
        };
        hit.distance = distance;
        hit.set_face_normal(
            r,
            match self.axis {
                Axis::X => Vec3(1.0, 0.0, 0.0),
                Axis::Y => Vec3(0.0, 1.0, 0.0),
                Axis::Z => Vec3(0.0, 0.0, 1.0),
            },
        );
        Some(hit)
    }
}
