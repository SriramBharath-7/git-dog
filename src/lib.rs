//! Git Dog's independent, deterministic Rust pixel-art scene renderer.

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 144;
type Color = [u8; 4];
const fn c(r: u8, g: u8, b: u8) -> Color { [r, g, b, 255] }
const INK: Color = c(26, 24, 39);
const DARK: Color = c(15, 19, 31);
const ORANGE: Color = c(251, 172, 79);
const LIGHT: Color = c(255, 196, 113);
const SHADE: Color = c(210, 115, 55);
const CREAM: Color = c(255, 237, 206);
const BLUSH: Color = c(255, 138, 153);
const RED: Color = c(219, 65, 79);
const RED_LIGHT: Color = c(255, 101, 103);
const GRASS: Color = c(79, 154, 80);
const GRASS_LIGHT: Color = c(139, 202, 78);
const GRASS_DARK: Color = c(45, 105, 65);
const WOOD: Color = c(156, 91, 70);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood { Playful, Idle, Sleepy }

pub struct Canvas { rgba: Vec<u8> }
impl Canvas {
    fn new() -> Self {
        let mut rgba = vec![0; WIDTH * HEIGHT * 4];
        for pixel in rgba.chunks_mut(4) { pixel.copy_from_slice(&c(24, 28, 44)); }
        Self { rgba }
    }
    pub fn bytes(&self) -> &[u8] { &self.rgba }
    pub fn upscale(&self, scale: u16) -> Vec<u8> {
        let k = usize::from(scale);
        let mut result = vec![0; WIDTH * HEIGHT * k * k * 4];
        for y in 0..HEIGHT * k {
            for x in 0..WIDTH * k {
                let src = ((y / k) * WIDTH + x / k) * 4;
                let dst = (y * WIDTH * k + x) * 4;
                result[dst..dst + 4].copy_from_slice(&self.rgba[src..src + 4]);
            }
        }
        result
    }
    fn point(&mut self, x: i32, y: i32, color: Color) {
        if (0..WIDTH as i32).contains(&x) && (0..HEIGHT as i32).contains(&y) {
            let at = (y as usize * WIDTH + x as usize) * 4;
            self.rgba[at..at + 4].copy_from_slice(&color);
        }
    }
    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color) {
        for yy in y.max(0)..(y + h).min(HEIGHT as i32) {
            for xx in x.max(0)..(x + w).min(WIDTH as i32) { self.point(xx, yy, color); }
        }
    }
    fn ellipse(&mut self, x: i32, y: i32, rx: i32, ry: i32, color: Color) {
        if rx <= 0 || ry <= 0 { return; }
        let (a, b) = (i64::from(rx), i64::from(ry));
        for yy in (y - ry).max(0)..=(y + ry).min(HEIGHT as i32 - 1) {
            for xx in (x - rx).max(0)..=(x + rx).min(WIDTH as i32 - 1) {
                let (dx, dy) = (i64::from(xx - x), i64::from(yy - y));
                if dx * dx * b * b + dy * dy * a * a <= a * a * b * b {
                    self.point(xx, yy, color);
                }
            }
        }
    }
    fn triangle(&mut self, a: (i32, i32), b: (i32, i32), d: (i32, i32), color: Color) {
        let (xmin, xmax) = (a.0.min(b.0).min(d.0).max(0), a.0.max(b.0).max(d.0).min(WIDTH as i32 - 1));
        let (ymin, ymax) = (a.1.min(b.1).min(d.1).max(0), a.1.max(b.1).max(d.1).min(HEIGHT as i32 - 1));
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                let p = (x, y);
                let edges = [cross(p, a, b), cross(p, b, d), cross(p, d, a)];
                if edges.iter().all(|v| *v >= 0) || edges.iter().all(|v| *v <= 0) {
                    self.point(x, y, color);
                }
            }
        }
    }
    fn line(&mut self, (mut x, mut y): (i32, i32), to: (i32, i32), width: i32, color: Color) {
        let (dx, dy) = ((to.0 - x).abs(), -(to.1 - y).abs());
        let (sx, sy) = (if x < to.0 { 1 } else { -1 }, if y < to.1 { 1 } else { -1 });
        let mut err = dx + dy;
        loop {
            self.rect(x - width / 2, y - width / 2, width, width, color);
            if (x, y) == to { break; }
            let e = 2 * err;
            if e >= dy { err += dy; x += sx; }
            if e <= dx { err += dx; y += sy; }
        }
    }
}
fn cross(p: (i32,i32), a: (i32,i32), b: (i32,i32)) -> i64 {
    i64::from(p.0 - a.0) * i64::from(b.1 - a.1) - i64::from(p.1 - a.1) * i64::from(b.0 - a.0)
}

/// Same frame, mood, and frame count always produce identical pixel data.
pub fn render(frame: u16, count: u16, mood: Mood) -> Canvas {
    let phase = if count == 0 { 0 } else { usize::from(frame) * 8 / usize::from(count) };
    let mut p = Canvas::new();
    background(&mut p, phase);
    house(&mut p);
    ground(&mut p);
    dog(&mut p, phase % 8, mood);
    toys(&mut p);
    calendar(&mut p);
    progress(&mut p, phase % 8);
    p
}

fn background(p: &mut Canvas, phase: usize) {
    for y in (16..110).step_by(17) {
        let offset = if y / 17 % 2 == 0 { 0 } else { 22 };
        for x in (0..270).step_by(43) {
            p.rect(x + offset - 8, y, 39, 12, c(39, 40, 60));
            p.rect(x + offset - 6, y + 1, 31, 1, c(46, 45, 66));
        }
    }
    for i in 0..9 {
        if i % 3 == phase % 3 { p.rect((i as i32) * 31 + 5, (i as i32) * 9 + 13, 2, 2, c(93, 86, 123)); }
    }
    for x in (3..91).step_by(11) {
        p.rect(x, 15, 3, 13, GRASS_DARK);
        p.ellipse(x + 3, 25, 7, 4, GRASS);
        p.rect(x + 1, 22, 4, 2, GRASS_LIGHT);
    }
    for x in (224..256).step_by(10) {
        p.ellipse(x, 23, 8, 5, GRASS);
    }
}
fn house(p: &mut Canvas) {
    p.rect(15, 55, 80, 57, INK);
    p.rect(18, 58, 74, 51, WOOD);
    for x in (24..92).step_by(13) { p.rect(x, 60, 2, 45, c(193, 122, 73)); }
    p.triangle((8,57), (53,21), (101,57), INK);
    p.triangle((13,54), (53,26), (96,54), c(134, 44, 60));
    p.triangle((20,52), (53,29), (90,52), RED);
    p.line((17,52), (53,26), 2, RED_LIGHT);
    p.ellipse(55, 88, 23, 24, INK);
    p.ellipse(55, 91, 18, 23, DARK);
    p.ellipse(54, 43, 5, 4, CREAM);
    for (x,y) in [(49,37),(54,35),(60,37)] { p.ellipse(x,y,2,2,CREAM); }
    p.rect(30, 68, 10, 2, INK);
    p.rect(32, 70, 7, 10, INK);
    p.rect(33, 71, 5, 8, c(255, 205, 92));
    p.rect(30, 81, 12, 3, c(102, 52, 58));
    p.rect(30, 107, 55, 4, c(202, 131, 75));
}
fn ground(p: &mut Canvas) {
    p.rect(0, 107, 256, 22, GRASS_DARK);
    p.rect(0, 112, 256, 11, GRASS);
    p.rect(0, 112, 256, 3, GRASS_LIGHT);
    for x in (3..254).step_by(9) {
        p.rect(x, 107 + x % 4, 3, 5, GRASS);
        p.rect(x + 2, 105 + x % 3, 2, 6, GRASS_LIGHT);
    }
    for x in (5..251).step_by(17) { p.rect(x, 120, 6, 2, GRASS_LIGHT); }
    p.rect(0, 127, 256, 3, c(66, 60, 83));
}
fn dog(p: &mut Canvas, phase: usize, mood: Mood) {
    let sway = match mood {
        Mood::Playful => [-4, -2, 2, 5, 4, 2, -2, -5][phase],
        Mood::Idle => [-1, 0, 1, 2, 1, 0, -1, -2][phase],
        Mood::Sleepy => 0,
    };
    let bob = if mood == Mood::Playful && (2..=4).contains(&phase) { -2 } else { 0 };
    // Animated tail underneath the body.
    p.ellipse(107 + sway, 91 + bob, 16, 21, INK);
    p.ellipse(107 + sway, 89 + bob, 13, 18, SHADE);
    p.ellipse(104 + sway, 84 + bob, 10, 14, ORANGE);
    p.ellipse(101 + sway, 77 + bob, 7, 9, CREAM);
    // Sitting body and cream belly.
    p.ellipse(141, 93 + bob, 38, 29, INK);
    p.ellipse(141, 91 + bob, 35, 26, SHADE);
    p.ellipse(138, 88 + bob, 31, 23, ORANGE);
    p.ellipse(157, 102 + bob, 17, 21, CREAM);
    p.ellipse(122, 110 + bob, 17, 11, CREAM);
    // Shiba/Corgi ears: one standing, one floppy.
    p.triangle((148, 56 + bob), (163, 23 + bob), (181, 56 + bob), INK);
    p.triangle((153, 54 + bob), (163, 29 + bob), (176, 55 + bob), ORANGE);
    p.triangle((161, 50 + bob), (164, 34 + bob), (171, 52 + bob), BLUSH);
    p.ellipse(120, 56 + bob, 23, 12, INK);
    p.ellipse(120, 54 + bob, 20, 10, ORANGE);
    p.ellipse(113, 62 + bob, 9, 9, SHADE);
    p.ellipse(115, 61 + bob, 5, 7, BLUSH);
    // Oversized cheerful face.
    p.ellipse(148, 68 + bob, 40, 31, INK);
    p.ellipse(149, 67 + bob, 37, 28, ORANGE);
    p.ellipse(137, 58 + bob, 24, 14, LIGHT);
    p.ellipse(160, 82 + bob, 29, 14, CREAM);
    p.ellipse(131, 83 + bob, 17, 11, CREAM);
    p.ellipse(176, 78 + bob, 12, 13, CREAM);
    p.ellipse(148, 87 + bob, 25, 9, CREAM);
    if mood == Mood::Sleepy {
        p.line((126,67+bob), (138,67+bob), 3, INK);
        p.line((164,66+bob), (175,66+bob), 3, INK);
    } else {
        p.line((127,68+bob), (132,62+bob), 4, INK);
        p.line((132,62+bob), (138,66+bob), 4, INK);
        p.line((165,65+bob), (170,59+bob), 4, INK);
        p.line((170,59+bob), (175,63+bob), 4, INK);
    }
    p.ellipse(125, 78 + bob, 4, 3, BLUSH);
    p.ellipse(182, 74 + bob, 5, 3, BLUSH);
    p.ellipse(156, 72 + bob, 4, 3, INK);
    p.line((156,75+bob), (161,78+bob), 2, INK);
    if mood == Mood::Sleepy {
        p.line((159,81+bob),(170,81+bob),2,INK);
    } else {
        p.ellipse(165, 83 + bob, 10, 10, INK);
        p.ellipse(166, 87 + bob, 7, 6, BLUSH);
        p.rect(159, 76 + bob, 7, 2, CREAM);
    }
    // Red code-symbol bandana, separate from the muzzle.
    p.triangle((139,92+bob), (183,91+bob), (164,118+bob), INK);
    p.triangle((143,94+bob), (178,94+bob), (164,114+bob), RED);
    p.line((145,96+bob), (176,96+bob), 3, RED_LIGHT);
    p.line((152,101+bob),(148,105+bob),2,CREAM);
    p.line((148,105+bob),(152,109+bob),2,CREAM);
    p.line((169,101+bob),(173,105+bob),2,CREAM);
    p.line((173,105+bob),(169,109+bob),2,CREAM);
    p.line((162,100+bob),(158,110+bob),2,CREAM);
    // Four visible tiny paws.
    for (x,y) in [(111,114),(144,114),(166,113)] {
        p.rect(x, y + bob, 14, 8, INK);
        p.rect(x+2, y-2 + bob, 10, 8, CREAM);
        p.rect(x+5, y+4 + bob, 1, 2, c(232,188,159));
    }
    if mood == Mood::Sleepy {
        for (x,y) in [(189,45),(203,33)] {
            p.line((x,y),(x+6,y),2,CREAM);
            p.line((x+6,y),(x,y+6),2,CREAM);
            p.line((x,y+6),(x+6,y+6),2,CREAM);
        }
    } else if mood == Mood::Playful {
        let x = 188 + (phase % 3) as i32;
        p.rect(x, 42, 4, 7, c(255,207,104));
        p.rect(x+7, 46, 3, 4, c(255,207,104));
    }
}
fn toys(p: &mut Canvas) {
    p.rect(204,110,24,7,INK);
    p.ellipse(203,111,7,7,INK);
    p.ellipse(229,111,7,7,INK);
    p.rect(204,110,25,5,CREAM);
    p.ellipse(203,110,4,5,CREAM);
    p.ellipse(229,110,4,5,CREAM);
    p.ellipse(239,119,9,9,INK);
    p.ellipse(239,118,7,7,RED);
    p.rect(237,114,5,2,CREAM);
}
fn calendar(p: &mut Canvas) {
    p.rect(196,34,54,69,INK);
    p.rect(199,37,48,63,WOOD);
    p.rect(202,40,42,56,c(24,28,44));
    p.rect(207,44,23,3,CREAM);
    p.rect(207,49,27,2,c(79,74,102));
    let shades = [c(39,66,60),c(53,119,77),c(83,164,87),c(142,214,90)];
    for row in 0..5 {
        for col in 0..7 {
            let level = (col * 7 + row * 11 + col * row) % 4;
            p.rect(207 + col as i32 * 5, 56 + row as i32 * 7, 4, 5, shades[level]);
        }
    }
    p.rect(196,100,54,3,c(197,126,74));
}
fn progress(p: &mut Canvas, phase: usize) {
    p.rect(0,130,256,14,DARK);
    p.rect(0,130,256,1,c(96,82,132));
    for i in 0..20 {
        let color = if i < 11 { c(174,113,220) } else { c(62,55,86) };
        p.rect(9+i*5,135,4,3,color);
    }
    for i in 0..8 {
        p.rect(170+i*9,135,6,3,if i as usize == phase { GRASS_LIGHT } else { GRASS_DARK });
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dimensions_and_opacity() {
        let image = render(0,24,Mood::Playful);
        assert_eq!(image.bytes().len(), WIDTH*HEIGHT*4);
        assert!(image.bytes().chunks(4).all(|p|p[3]==255));
    }
    #[test]
    fn animated() {
        assert_ne!(render(0,24,Mood::Playful).bytes(),render(12,24,Mood::Playful).bytes());
    }
    #[test]
    fn deterministic() {
        assert_eq!(render(3,24,Mood::Sleepy).bytes(),render(3,24,Mood::Sleepy).bytes());
    }
    #[test]
    fn upscales_without_blur() {
        let image = render(0,24,Mood::Idle);
        let doubled = image.upscale(2);
        assert_eq!(doubled.len(),WIDTH*HEIGHT*16);
        assert_eq!(&doubled[..4],&image.bytes()[..4]);
        assert_eq!(&doubled[4..8],&image.bytes()[..4]);
    }
    #[test]
    fn zero_frame_count_is_safe() {
        assert_eq!(render(0,0,Mood::Idle).bytes().len(), WIDTH*HEIGHT*4);
    }
}
