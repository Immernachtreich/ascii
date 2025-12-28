use std::{ cmp::{ min }, error::Error };

use console::{ Term, style };
use image::{ GenericImageView, ImageReader };

const BRIGHTNESS: &str = "Ñ@#W$9876543210?!abc;:+=-,._      ";

fn main() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();
    let (_, columns) = term.size();
    let width = ((columns / 2) as f32).floor() as u32;

    // let img = ImageReader::open("assets/me.jpg")?.decode()?;
    // let img = ImageReader::open("assets/strawberry.jpg")?.decode()?;
    let img = ImageReader::open("assets/cat.jpg")?.decode()?;
    // let img = ImageReader::open("assets/skull.jpg")?.decode()?;
    // let img = ImageReader::open("assets/pixel.png")?.decode()?;

    let target_width = min(width, img.width());

    let img = img.resize(target_width, target_width, image::imageops::FilterType::Nearest);

    // Convert brightness string to a vector for easy access via indexing
    let ramp: Vec<char> = BRIGHTNESS.chars().collect();
    let ramp_len = ramp.len() as f32;

    let mut last_y = 0;

    for (_, y, rgba) in img.pixels() {
        if y != last_y {
            println!();
            last_y = y;
        }

        let r = rgba[0] as f32;
        let g = rgba[1] as f32;
        let b = rgba[2] as f32;

        let luminance = (r + g + b) / 3.0;
        let normalized = luminance / 255.0;

        let idx = (normalized * (ramp_len - 1.0)).round().clamp(0.0, ramp_len - 1.0) as usize;

        // Invert so darker pixels use denser characters
        let ch = ramp[ramp.len() - 1 - idx];

        print!("{} ", style(ch).true_color(r as u8, g as u8, b as u8));
    }

    Ok(())
}
