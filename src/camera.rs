use std::fmt;
use crate::camera_brand::CameraBrand;
use CameraBrand::*;

pub struct Camera {
    model: String,
    brand: CameraBrand,
    price_per_hour: f32,
}

impl Camera {
    pub fn new(model: String, brand: CameraBrand, price_per_hour: f32) -> Self {
        Self {
            model,
            brand,
            price_per_hour
        }
    }

    pub fn get_model(&self) -> &String {
        &self.model
    }
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
        write!(f, "Model: {}\n   Brand: {}\n   Price Per Hour: {}$", self.model, self.brand, self.price_per_hour)
    }
}