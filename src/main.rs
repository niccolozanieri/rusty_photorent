mod camera;
mod lens;
mod photorent_shop;
mod camera_brand;
mod lens_brand;

use crate::camera_brand::CameraBrand;
use crate::camera::Camera;
use crate::lens::Lens;
use crate::lens_brand::LensBrand;

fn main() {
    let a7 = Camera::new("a7".to_string(), CameraBrand::SONY, 20f32);
    let a7ii = Camera::new("a7ii".to_string(), CameraBrand::SONY, 25f32);
    let sony_g85 = Lens::new("Series G".to_string(), LensBrand::SONY, 85u8, 32f32, 1.8f32, 10f32);
    let sony_50 = Lens::new("Series G".to_string(), LensBrand::SONY, 50u8, 32f32, 1.8f32, 10f32);

}
