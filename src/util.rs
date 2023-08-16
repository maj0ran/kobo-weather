use crate::math::{Mat2, Vec2};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

pub struct FontSetting {
    pub name: &'static str,
    pub size: f32,
    pub saturation: f32,
}

pub struct Line {
    p1: Vec2<i16>,
    p2: Vec2<i16>,
    thickness: u16,
}

impl Line {
    pub fn n(p1: Vec2<i16>, p2: Vec2<i16>, thickness: u16) -> Line {
        Line { p1, p2, thickness }
    }
    pub fn new(p1: Vec2<i16>, p2: Vec2<i16>, thickness: i16) -> Vec<Vec2<i16>> {
        // bounding box
        let x_min = p1.x.min(p2.x) - thickness / 2;
        let x_max = p1.x.max(p2.x) + thickness / 2;
        let y_min = p1.y.min(p2.y) - thickness / 2;
        let y_max = p1.y.max(p2.y) + thickness / 2;

        let mut pixels = Vec::new();

        let ba = Vec2::<f32>::from(p2 - p1);
        let l = ba.len();
        let d = ba.norm();
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let p = Vec2::new(x, y);
                let q = Vec2::<f32>::from(p) - (Vec2::<f32>::from(p1 + p2) * 0.5);
                let q = Mat2::<f32>::new((d.x, -d.y, d.y, d.x)) * q;
                let q = q.abs() - (Vec2::new(l, thickness as f32) * 0.5);
                let dist = q.max(Vec2::new(0.0, 0.0)).len() + (q.x.max(q.y).min(0.0));
                if dist <= 0.0 {
                    pixels.push(p);
                }
            }
        }
        pixels
    }
}

pub struct Rectangle {
    width: u16,
    height: u16,
    saturation: f32,
}
