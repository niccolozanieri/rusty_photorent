mod camera;
mod lens;
mod photorent_shop;
mod camera_brand;
mod lens_brand;

use crate::camera_brand::CameraBrand;
use crate::camera::Camera;
use crate::lens::Lens;
use crate::lens_brand::LensBrand;
use std::io;
use std::io::prelude::*;

fn main() {
    let a7 = Camera::new("a7".to_string(), CameraBrand::SONY, 20f32);
    let a7ii = Camera::new("a7ii".to_string(), CameraBrand::SONY, 25f32);
    let sony_g85 = Lens::new("Series G".to_string(), LensBrand::SONY, 85u8, 32f32, 1.8f32, 10f32);
    let sony_50g = Lens::new("Series G".to_string(), LensBrand::SONY, 50u8, 32f32, 1.8f32, 10f32);

    let cameras = vec![a7, a7ii];
    let lenses = vec![sony_50g, sony_g85];

    println!("WELCOME TO RUSTY-RENT!!");
    print!("\n\nEnter 'c' to show our cameras list, 'l' to show the lenses list: ");
    io::stdout().flush().unwrap();
    let mut list_choice = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut list_choice).expect("TODO: panic message");

    println!("\n");

    let selected_list = match list_choice.as_str() {
        "c\n" => {
            for (i, camera) in cameras.iter().enumerate() { println!("{}. {}", i + 1, camera) };
        }
        "l\n" =>  {
            for (i, lens) in lenses.iter().enumerate() { println!("{}. {}", i + 1, lens) }
        }

        _ => println!("Error."),
    };
}
