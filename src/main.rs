use clap::Parser;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Parser)]
#[command(version, about)]
struct Config {
    /// Output image width
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 250)]
    width: u16,

    /// Output image height
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 250)]
    height: u16,

    /// Output file path
    #[arg(long, short = 'o', default_value_t = String::from("output.ppm"))]
    output_path: String,
}

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let config = Config::parse();

    let magic_number = "P6";
    let width = config.width as usize;
    let height = config.height as usize;
    let maxval = 255u8;
    let output_path = config.output_path;

    let header = format!("{magic_number}\n{width} {height}\n{maxval}\n");

    let mut bytes = Vec::new();
    bytes.extend_from_slice(header.as_bytes());

    let mut raster: Vec<u8> = Vec::with_capacity(width * height * 3);

    for y in 0..height {
        for x in 0..width {
            let x = (x as f32 / (width - 1) as f32 * 255.0).round() as u8;
            let y = (y as f32 / (height - 1) as f32 * 255.0).round() as u8;

            let r = x.wrapping_mul(x);
            let g = x.wrapping_add(y);
            let b = x.wrapping_sub(y);
            raster.extend_from_slice(&[r, g, b]);
        }
    }

    bytes.extend_from_slice(&raster);

    fs::write(&output_path, bytes).expect("Something went wrong");
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let elapsed = start.abs_diff(end);
    println!("Wrote {width}x{height} image to \"{output_path}\" in {elapsed:.2?}",);
}