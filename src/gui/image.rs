use crate::{math::Vec2, util::Color};
use image::{imageops::FilterType, io::Reader as ImageReader};
use std::path::Path;

use super::gui::Widget;

/***
 * GUI-Element that shows an Image on the E-Ink Display
 ***/
#[derive(Debug)]
pub struct Image {
    pos: Option<Vec2<i16>>,
    width: i16,
    height: i16,
    pixels: Vec<Color>,
}

impl Image {
    pub fn new(filename: &str, scale: f32) -> Box<Image> {
        let file = Path::new(filename);
        println!("{}", filename);
        let data = ImageReader::open(file).unwrap().decode().unwrap();
        let w: f32 = data.width() as f32;
        let h: f32 = data.height() as f32;

        let data = data.resize((w * scale) as u32, (h * scale) as u32, FilterType::Gaussian);
        let width = data.width() as i16;
        let height = data.height() as i16;

        // turn into grayscale and convert transparent background to white
        let mut img = data.to_luma_alpha8();
        let mut pixels: Vec<Color> = Vec::with_capacity((width * height) as usize);

        // draw the pixels only in memory. Drawing on screen hapens when draw() is called.
        // Not really a fan of this approach because of copying the data, but I didn't figure out
        // how to do it more elegant yet, as I don't want let the GUI element call the draw
        // function but rather just return the pixel data.
        for px in img.pixels_mut() {
            let val = 255 - (px.0[0] * px.0[1]);
            pixels.push(Color::new(val, val, val))
        }

        Box::new(Image {
            pos: None,
            width,
            height,
            pixels,
        })
    }
}

impl Widget for Image {
    fn get_width(&self) -> i16 {
        self.width
    }

    fn get_height(&self) -> i16 {
        self.height
    }

    fn get_pos(&self) -> Option<Vec2<i16>> {
        self.pos
    }

    fn get_pixel_data(&self) -> &Vec<Color> {
        &self.pixels
    }

    fn set_pos(&mut self, pos: Vec2<i16>) {
        self.pos = Some(pos);
    }
}
