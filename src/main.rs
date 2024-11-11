#![feature(path_file_prefix)]

use clap::Parser;
use image::{ImageReader, Rgba};
use imageproc::map::*;
use std::path::Path;

#[derive(Parser)]
struct Cli {
    path: String,
}

fn main() {
    let args = Cli::parse();
    let path = Path::new(&args.path);
    let image = ImageReader::open(path).expect("Path should be an image.");
    let binding = image.decode().unwrap().to_rgba8();
    let rgb = map_colors(&binding, |p| { Rgba([p[3], p[1], p[0], p[0]]) });
    let new_file = path.file_prefix().unwrap().to_str().unwrap().to_string() + "_new" + ".png";
    let save_path = path.with_file_name(new_file);
    println!("{:?}", save_path);
    rgb.save_with_format(save_path, image::ImageFormat::Png).unwrap();
}
