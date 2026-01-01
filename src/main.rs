use std::{ cmp::min, error::Error, fs, mem, process::Command, thread, time::{ Duration, Instant } };

use console::{ Term, style };
use image::{ DynamicImage, GenericImageView, ImageReader };

const BRIGHTNESS: &str = "Ñ@#W$9876543210?!abc;:+=-,._      ";
const FPS: u64 = 10;

mod guards;

use crate::guards::terminal_guard::TerminalGuard;

struct ASCIIPixel(u8, u8, u8, char);

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

        let luminance = (r + g + b) / 3.0;
        let normalized = luminance / 255.0;

        let idx = (normalized * (ramp_len - 1.0)).round().clamp(0.0, ramp_len - 1.0) as usize;

        // Invert so darker pixels use denser characters
        let ch = ramp[ramp.len() - 1 - idx];

        let ascii_pixel = ASCIIPixel(r as u8, g as u8, b as u8, ch);

        line_pixels.push(ascii_pixel);
    }

    if !line_pixels.is_empty() {
        ascii_img.push(line_pixels);
    }

    ascii_img
}

fn draw_image(pixels: Vec<Vec<ASCIIPixel>>) -> Result<(), Box<dyn Error>> {
    let terminal = Term::stdout();

    terminal.move_cursor_to(0, 0)?;

    let mut frame = String::new();

    for row in pixels {
        for px in row {
            // let image_pixel = format!("{}{}", px.3, px.3);
            let image_pixel = &style(format!("{}{}", px.3, px.3))
                .true_color(px.0, px.1, px.2)
                .to_string();
            // let image_pixel = style("  ")
            //     .bg(console::Color::TrueColor(px.r, px.g, px.b))
            //     .to_string();

            frame.push_str(&image_pixel);
        }

        terminal.write_line(&mem::take(&mut frame))?;
    }
    terminal.flush()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    fs::remove_dir_all("assets/frames")?;
    fs::create_dir("assets/frames")?;

    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg("assets/sample_video.mp4")
        .arg("-vf")
        .arg(format!("fps={},scale=120:-1", FPS))
        .arg("assets/frames/frame_%05d.jpg")
        .status()?;

    if !status.success() {
        return Err("ffmpeg failed".into());
    }

    let terminal_guard = TerminalGuard::new();

    terminal_guard.enter_alternate_screen()?;

    let frames_dir = fs::read_dir("assets/frames")?;
    let frame_duration = Duration::from_secs_f32(1.0 / (FPS as f32));

    for (_, frame) in frames_dir.enumerate() {
        let start_time = Instant::now();

        let img = ImageReader::open(frame.unwrap().path())?.decode()?;
        let img = resize_image(img);
        let image_pixels = convert_to_ascii(img);

        draw_image(image_pixels)?;

        let elapsed_time = start_time.elapsed();
        if frame_duration > elapsed_time {
            thread::sleep(frame_duration - elapsed_time);
        }
    }

    terminal_guard.leave_alternate_screen()?;

    Ok(())
}
