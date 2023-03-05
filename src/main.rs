use crate::camera_brand::CameraBrand;

mod camera;
mod lens;
mod photorent_shop;
mod camera_brand;
mod lens_brand;

fn main() {
    let camera = camera::Camera {
        model: String::from("a7"),
        brand: CameraBrand::SONY,
        ..Default::default()
    };

    let lens = lens::Lens::default();

    println!("{}", camera);
    println!("{}", lens);
}
