use std::{ cmp::min, error::Error, mem };

use console::{ Term, style };
use image::{ DynamicImage, GenericImageView, ImageReader };

const BRIGHTNESS: &str = "Ñ@#W$9876543210?!abc;:+=-,._      ";

struct ASCIIPixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    ch: char,
}

fn resize_image(img: DynamicImage) -> DynamicImage {
    let term = Term::stdout();
    let (_, columns) = term.size();
    let width = ((columns / 2) as f32).floor() as u32;

    let target_width = min(width, img.width());

    // Image is already smaller than the target image. No resizing needed.
    if img.width() <= target_width {
        return img;
    }

    let img = img.resize(target_width, target_width, image::imageops::FilterType::Nearest);

    img
}

fn convert_to_ascii(img: DynamicImage) -> Vec<Vec<ASCIIPixel>> {
    // Convert brightness string to a vector for easy access via indexing
    let ramp: Vec<char> = BRIGHTNESS.chars().collect();
    let ramp_len = ramp.len() as f32;

    let mut ascii_img: Vec<Vec<ASCIIPixel>> = Vec::new();
    let mut line_pixels: Vec<ASCIIPixel> = Vec::new();

    let mut last_y: u32 = 0;

    for (_, y, rgba) in img.pixels() {
        if y != last_y {
            ascii_img.push(mem::take(&mut line_pixels));
            last_y = y;
        }

        let r = rgba[0] as f32;
        let g = rgba[1] as f32;
        let b = rgba[2] as f32;
        let a = rgba[3];

        let luminance = (r + g + b) / 3.0;
        let normalized = luminance / 255.0;

        let idx = (normalized * (ramp_len - 1.0)).round().clamp(0.0, ramp_len - 1.0) as usize;

        // Invert so darker pixels use denser characters
        let ch = ramp[ramp.len() - 1 - idx];

        let ascii_pixel = ASCIIPixel {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a: a,
            ch,
        };

        line_pixels.push(ascii_pixel);
    }

    if !line_pixels.is_empty() {
        ascii_img.push(line_pixels);
    }

    ascii_img
}

fn draw_image(pixels: Vec<Vec<ASCIIPixel>>) -> Result<(), Box<dyn Error>> {
    let terminal = Term::stdout();

    terminal.clear_screen()?;

    let mut frame = String::new();

    for row in pixels {
        for px in row {
            // Skip pixel if its transparent
            if px.a == 0 {
                frame.push(' ');
                continue;
            }

            frame.push_str(
                &style(format!("{}{}", px.ch, px.ch)).true_color(px.r, px.g, px.b).to_string()
            );
        }

        terminal.write_line(&mem::take(&mut frame))?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open("assets/strawberry.jpg")?.decode()?;

    let img = resize_image(img);
    let image_pixels = convert_to_ascii(img);
    draw_image(image_pixels)?;

    Ok(())
}
