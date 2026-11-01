//! Layered pixel sprites extracted from the approved Git Dog concept art.
//! Rust performs deterministic compositing, frame timing, and output encoding.
//!
//! The source artwork is stored as compressed indexed-color text assets.
//! This keeps the original expression and hand-pixel details, rather than
//! attempting to redraw them from overlapping ellipses and rectangles.

use base64::{engine::general_purpose::STANDARD, Engine as _};
use flate2::read::ZlibDecoder;
use std::io::Read;
use std::sync::OnceLock;

pub const WIDTH: usize = 300;
pub const HEIGHT: usize = 127;
const DOG_X: i32 = 97;
const DOG_Y: i32 = 11;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Playful,
    Idle,
    Sleepy,
}

struct Sprite {
    width: usize,
    height: usize,
    rgba: Vec<u8>,
}

struct Assets {
    scene: Sprite,
    puppy: Sprite,
    tail: Sprite,
}

static ASSETS: OnceLock<Assets> = OnceLock::new();

fn assets() -> &'static Assets {
    ASSETS.get_or_init(|| Assets {
        scene: decode(include_str!("../assets/scene.b64")),
        puppy: decode(include_str!("../assets/puppy.b64")),
        tail: decode(include_str!("../assets/tail.b64")),
    })
}

/// The format is zlib(base64(LE-width, LE-height, palette-count,
/// palette RGB bytes, one index per pixel)). Index 255 is transparent.
fn decode(encoded: &str) -> Sprite {
    let packed = STANDARD.decode(encoded.trim()).expect("valid embedded base64 artwork");
    let mut bytes = Vec::new();
    ZlibDecoder::new(packed.as_slice())
        .read_to_end(&mut bytes)
        .expect("valid compressed artwork");
    assert!(bytes.len() >= 5, "truncated artwork header");
    let width = usize::from(u16::from_le_bytes([bytes[0], bytes[1]]));
    let height = usize::from(u16::from_le_bytes([bytes[2], bytes[3]]));
    let count = usize::from(bytes[4]);
    assert!(count > 0 && count < 255, "invalid palette");
    let start = 5 + count * 3;
    assert_eq!(bytes.len(), start + width * height, "invalid artwork dimensions");
    let palette = &bytes[5..start];
    let mut rgba = vec![0; width * height * 4];
    for (i, index) in bytes[start..].iter().copied().enumerate() {
        if index == 255 {
            continue;
        }
        let n = usize::from(index);
        assert!(n < count, "invalid pixel palette index");
        rgba[i * 4..i * 4 + 3].copy_from_slice(&palette[n * 3..n * 3 + 3]);
        rgba[i * 4 + 3] = 255;
    }
    Sprite { width, height, rgba }
}

pub struct Canvas {
    rgba: Vec<u8>,
}

impl Canvas {
    pub fn bytes(&self) -> &[u8] {
        &self.rgba
    }

    /// Nearest-neighbor scale: keeps hand-made pixel edges crisp.
    pub fn upscale(&self, scale: u16) -> Vec<u8> {
        assert!((1..=5).contains(&scale));
        let factor = usize::from(scale);
        let output_width = WIDTH * factor;
        let mut result = vec![0; output_width * HEIGHT * factor * 4];
        for y in 0..HEIGHT * factor {
            for x in 0..output_width {
                let src = ((y / factor) * WIDTH + x / factor) * 4;
                let dst = (y * output_width + x) * 4;
                result[dst..dst + 4].copy_from_slice(&self.rgba[src..src + 4]);
            }
        }
        result
    }

    fn sprite(&mut self, layer: &Sprite, x: i32, y: i32) {
        for sy in 0..layer.height {
            let dy = y + sy as i32;
            if !(0..HEIGHT as i32).contains(&dy) {
                continue;
            }
            for sx in 0..layer.width {
                let dx = x + sx as i32;
                if !(0..WIDTH as i32).contains(&dx) {
                    continue;
                }
                let from = (sy * layer.width + sx) * 4;
                if layer.rgba[from + 3] == 0 {
                    continue;
                }
                let to = (dy as usize * WIDTH + dx as usize) * 4;
                self.rgba[to..to + 4].copy_from_slice(&layer.rgba[from..from + 4]);
            }
        }
    }

    fn dot(&mut self, x: i32, y: i32, color: [u8; 4]) {
        if !(0..WIDTH as i32).contains(&x) || !(0..HEIGHT as i32).contains(&y) {
            return;
        }
        let i = (y as usize * WIDTH + x as usize) * 4;
        self.rgba[i..i + 4].copy_from_slice(&color);
    }

    fn sparkle(&mut self, x: i32, y: i32, radius: i32) {
        let color = [255, 212, 115, 255];
        for distance in -radius..=radius {
            self.dot(x + distance, y, color);
            self.dot(x, y + distance, color);
        }
    }

    fn zzz(&mut self, x: i32, y: i32, scale: i32) {
        let pale = [232, 218, 252, 255];
        for i in 0..=scale {
            self.dot(x + i, y, pale);
            self.dot(x + scale - i, y + i, pale);
            self.dot(x + i, y + scale, pale);
        }
    }
}

/// Draws one complete frame. Looping is intentional: frame 0 and the final
/// frame have compatible tail positions without harsh wraparound.
pub fn render(frame: u16, total_frames: u16, mood: Mood) -> Canvas {
    let a = assets();
    let mut canvas = Canvas { rgba: a.scene.rgba.clone() };
    let phase = if total_frames == 0 {
        0
    } else {
        usize::from(frame % total_frames) * 8 / usize::from(total_frames)
    };
    let phase = phase.min(7);
    let tail_sway = match mood {
        Mood::Playful => [-2, -1, 0, 1, 2, 1, 0, -1][phase],
        Mood::Idle => [-1, 0, 0, 1, 1, 0, 0, -1][phase],
        Mood::Sleepy => 0,
    };
    let bob = match mood {
        Mood::Playful if [2, 3, 4].contains(&phase) => -1,
        Mood::Idle if [3, 4].contains(&phase) => -1,
        _ => 0,
    };
    // Layer order matters: tail BEHIND torso; all pixels from approved art.
    canvas.sprite(&a.tail, DOG_X + tail_sway, DOG_Y + bob);
    canvas.sprite(&a.puppy, DOG_X, DOG_Y + bob);

    match mood {
        Mood::Playful if phase == 2 || phase == 4 => canvas.sparkle(213, 39, 2),
        Mood::Sleepy => {
            canvas.zzz(210, 36, 4);
            canvas.zzz(220, 28, 5);
        }
        _ => {}
    }
    canvas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprites_are_separate_and_correct_size() {
        let a = assets();
        assert_eq!((a.scene.width, a.scene.height), (WIDTH, HEIGHT));
        assert_eq!((a.puppy.width, a.puppy.height), (96, 100));
        assert_eq!((a.tail.width, a.tail.height), (96, 100));
        assert!(a.tail.rgba.chunks(4).any(|pixel| pixel[3] == 0));
        assert!(a.tail.rgba.chunks(4).any(|pixel| pixel[3] == 255));
    }

    #[test]
    fn output_dimensions_and_opaque_background() {
        let image = render(0, 24, Mood::Idle);
        assert_eq!(image.bytes().len(), WIDTH * HEIGHT * 4);
        assert!(image.bytes().chunks(4).all(|pixel| pixel[3] == 255));
    }

    #[test]
    fn playful_animation_has_distinct_frames() {
        assert_ne!(render(0, 24, Mood::Playful).bytes(), render(9, 24, Mood::Playful).bytes());
    }

    #[test]
    fn same_frame_is_identical() {
        assert_eq!(render(2, 24, Mood::Playful).bytes(), render(2, 24, Mood::Playful).bytes());
    }

    #[test]
    fn scaling_duplicates_pixels() {
        let first = render(0, 24, Mood::Idle);
        let doubled = first.upscale(2);
        assert_eq!(doubled.len(), WIDTH * HEIGHT * 16);
        assert_eq!(&doubled[..4], &first.bytes()[..4]);
        assert_eq!(&doubled[4..8], &first.bytes()[..4]);
    }

    #[test]
    fn all_moods_render() {
        for mood in [Mood::Playful, Mood::Idle, Mood::Sleepy] {
            assert_eq!(render(0, 0, mood).bytes().len(), WIDTH * HEIGHT * 4);
        }
    }
}
