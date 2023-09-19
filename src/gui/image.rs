use crate::{math::UVec, util::Color};
use image::{imageops::FilterType, io::Reader as ImageReader, DynamicImage};
use std::path::Path;

use super::gui::{Position, Widget};

/***
 * GUI-Element that shows an Image on the E-Ink Display
 ***/
#[derive(Debug)]
pub struct Image {
    pos: UVec,
    width: u16,
    height: u16,

    data: DynamicImage,
}

impl Image {
    pub fn new(filename: &str, scale: f32, pos: Position) -> Box<Image> {
        let file = Path::new(filename);
        println!("{}", filename);
        let data = ImageReader::open(file).unwrap().decode().unwrap();
        let width = data.width() as f32;
        let height = data.height() as f32;

        let data = data.resize(
            (width * scale) as u32,
            (height * scale) as u32,
            FilterType::Gaussian,
        );

        // update width/height after resizing
        let width = data.width() as u16;
        let height = data.height() as u16;

        let w = Image {
            pos: UVec { x: 0, y: 0 },
            width,
            height,
            data,
        };

        let w = match pos {
            Position::Absolute(p) => w.set_pos_abs(p),
            Position::Relative(p) => w.set_pos_rel(p),
        };

        Box::new(w)
    }
}

impl Widget for Image {
    fn get_width(&self) -> u16 {
        self.width
    }

    fn get_height(&self) -> u16 {
        self.height
    }

    fn get_pos(&self) -> UVec {
        self.pos
    }

    fn get_pixel_data(&self) -> Vec<Color> {
        let mut pixels: Vec<Color> = Vec::with_capacity((self.width * self.height) as usize);
        let mut img = self.data.to_luma_alpha8();
        for px in img.pixels_mut() {
            let val = 255 - (px.0[0] * px.0[1]);
            pixels.push(Color::new(val, val, val))
        }
        pixels
    }

    fn set_pos(&mut self, pos: UVec) {
        self.pos = pos;
    }
}
