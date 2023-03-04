use std::fmt;

pub struct Lens {
    pub model: String,
    pub brand: String,
    pub focal_length: u8,
    pub minimum_aperture: f32,
    pub maximum_aperture: f32,
    pub price_per_hour: f32,
}


impl Default for Lens {
    fn default() -> Self {
        Self {
            model: String::from(""),
            brand: String::from(""),
            focal_length: 0,
            minimum_aperture: 0f32,
            maximum_aperture: 0f32,
            price_per_hour: 10f32
        }
    }
}

impl fmt::Display for Lens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model_brand_fl = format!(
            "Model: {}  Brand: {} Focal Length: {}",
            self.model,
            self.brand,
            self.focal_length
        );

        let min_max_price = format!(
            "Min Aperture:  {}  Max Aperture: {}  Price Per Hour: {}",
            self.minimum_aperture,
            self.maximum_aperture,
            self.price_per_hour
        );

        write!(f, "{}  {}", model_brand_fl, min_max_price)
    }
}