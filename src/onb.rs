use super::*;

pub struct ONB {
    axis: [Vec3; 3],
}

impl ONB {
    pub fn build_from_w(n: &Vec3) -> Self {
        let w = n.unit_vector();
        let a = if w.x().abs() > 0.9 {
            Vec3(0.0, 1.0, 0.0)
        } else {
            Vec3(1.0, 0.0, 0.0)
        };
        let v = w.cross(a).unit_vector();
        let u = w.cross(v);
        let axis = [u, v, w];
        Self { axis }
    }

    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn local(&self, a: &Vec3) -> Vec3 {
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }
}
