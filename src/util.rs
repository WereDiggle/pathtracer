pub trait Clamp {
    fn clam(&self, min: f64, max: f64) -> f64;
}

impl Clamp for f64 {
    fn clam(&self, min: f64, max: f64) -> f64 {
        self.max(min).min(max)
    }
}
