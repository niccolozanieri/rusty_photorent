use std::fmt;
use crate::camera_brand::CameraBrand;
use CameraBrand::*;

pub struct Camera {
    pub model: String,
    pub brand: CameraBrand,
    pub price_per_hour: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            model: String::from(""),
            brand: UNKNOWN,
            price_per_hour: 20f32,
        }
    }
}

impl fmt::Display for Camera {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Model: {}  Brand: {}  Price Per Hour: {}", self.model, self.brand, self.price_per_hour)
    }
}