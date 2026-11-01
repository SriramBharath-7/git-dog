use git_dog::{render, Mood, HEIGHT, WIDTH};
use std::{error::Error, fs::{self, File}, io::BufWriter, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::from("dist/git-dog.gif");
    let mut png_output = None;
    let mut mood = Mood::Playful;
    let mut frames = 24u16;
    let mut scale = 3u16;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => output = args.next().ok_or("--output requires a path")?,
            "--png" => png_output = Some(args.next().ok_or("--png requires a path")?),
            "--mood" => mood = match args.next().ok_or("--mood requires a value")?.as_str() {
                "playful" => Mood::Playful, "idle" => Mood::Idle,
                "sleepy" => Mood::Sleepy, other => return Err(format!("unknown mood: {other}").into()),
            },
            "--frames" => {
                frames = args.next().ok_or("--frames requires a number")?.parse()?;
                if !(2..=96).contains(&frames) { return Err("frames must be between 2 and 96".into()); }
            }
            "--scale" => {
                scale = args.next().ok_or("--scale requires a number")?.parse()?;
                if !(1..=5).contains(&scale) { return Err("scale must be between 1 and 5".into()); }
            }
            "-h" | "--help" => {
                println!("Git Dog - Rust pixel puppy");
                println!("Usage: git-dog [--output FILE.gif] [--png FILE.png] [--mood playful|idle|sleepy] [--frames 2..96] [--scale 1..5]");
                return Ok(());
            }
            other => return Err(format!("unknown argument: {other}").into()),
        }
    }
    let width = WIDTH as u16 * scale;
    let height = HEIGHT as u16 * scale;
    create_parent(&output)?;
    let mut encoder = gif::Encoder::new(BufWriter::new(File::create(&output)?), width, height, &[])?;
    encoder.set_repeat(gif::Repeat::Infinite)?;
    for i in 0..frames {
        let mut rgba = render(i, frames, mood).upscale(scale);
        let mut frame = gif::Frame::from_rgba_speed(width, height, &mut rgba, 10);
        frame.delay = 7;
        encoder.write_frame(&frame)?;
    }
    drop(encoder);
    println!("Wrote {output}");
    if let Some(path) = png_output {
        create_parent(&path)?;
        let mut encoder = png::Encoder::new(BufWriter::new(File::create(&path)?), width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&render(0, frames, mood).upscale(scale))?;
        println!("Wrote {path}");
    }
    Ok(())
}
fn create_parent(path: &str) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() { fs::create_dir_all(parent)?; }
    }
    Ok(())
}
