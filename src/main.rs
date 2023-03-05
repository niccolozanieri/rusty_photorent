mod camera;
mod lens;
mod photorent_shop;
mod camera_brand;
mod lens_brand;

fn main() {
    let camera = camera::Camera {
        model: String::from("a7"),
        brand: String::from("Sony"),
        ..Default::default()
    };

    let lens = lens::Lens::default();

    println!("{}", camera);
    println!("{}", lens);
}
