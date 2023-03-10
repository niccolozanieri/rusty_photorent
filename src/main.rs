mod camera;
mod lens;
mod photorent_shop;
mod camera_brand;
mod lens_brand;

use crate::camera_brand::CameraBrand;
use crate::camera::Camera;

fn main() {
    let a7 = camera::Camera::new("a7".to_string(), CameraBrand::SONY, 20f32);
}
