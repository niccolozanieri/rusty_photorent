use crate::lens_brand::LensBrand;
use std::fmt;

pub struct Lens {
    model: String,
    brand: LensBrand,
    focal_length: u8,
    min_aperture: f32,
    max_aperture: f32,
    price_per_hour: f32,
}

impl Lens {
    pub fn new(model: String, brand: LensBrand, focal_length: u8, min_aperture: f32, max_aperture: f32, price_per_hour: f32) -> Self {
        Self {
            model,
            brand,
            focal_length,
            min_aperture,
            max_aperture,
            price_per_hour
        }
    }
}


impl Default for Lens {
    fn default() -> Self {
        Self {
            model: String::from(""),
            brand: LensBrand::UNKNOWN,
            focal_length: 0,
            min_aperture: 0f32,
            max_aperture: 0f32,
            price_per_hour: 10f32
        }
    }
}

impl fmt::Display for Lens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model_brand_fl = format!(
            "Series: {}\n   Brand: {}\n   Focal Length: {}\n ",
            self.model,
            self.brand,
            self.focal_length
        );

        let min_max_price = format!(
            "Min Aperture:  {}\n   Max Aperture: {}\n   Price Per Hour: {}$ ",
            self.min_aperture,
            self.max_aperture,
            self.price_per_hour
        );

        write!(f, "{}  {}", model_brand_fl, min_max_price)
    }
}