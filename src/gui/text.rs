/***
 * GUI-Element that shows a Text on the E-Ink Display
 ***/

use crate::{
    math::Vec2,
    util::{Color, FontSetting},
};
use rusttype::{Font, PositionedGlyph, Scale};

use super::gui::Widget;

pub struct Text<'a> {
    // text: String
    pub pos: Option<Vec2<i16>>,
    pub data: Vec<PositionedGlyph<'a>>,
    pub width: i16,
    pub height: i16,
    pub font_info: FontSetting,
    pixels: Vec<Color>,
}

impl<'a> Text<'a> {
    pub fn new(text: &str, font_info: FontSetting) -> Box<Text<'a>> {
        let path = std::env::current_dir()
            .unwrap()
            .join("fonts/")
            .join(&font_info.name);
        let size = Scale::uniform(font_info.size);

        let font_type = std::fs::read(&path).unwrap();

        let font = Font::try_from_vec(font_type).unwrap();
        let v_metrics = font.v_metrics(size);

        let glyphs: Vec<_> = font
            .layout(text, size, rusttype::point(0.0, v_metrics.ascent))
            .collect();

        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as i32;
        let glyphs_width = glyphs.last().unwrap().pixel_bounding_box().unwrap().max.x;

        let mut pixels =
            vec![Color::new(255, 255, 255); glyphs_width as usize * glyphs_height as usize];

        // draw the pixels only in memory. Drawing on screen hapens when draw() is called.
        // Not really a fan of this approach because of copying the data, but I didn't figure out
        // how to do it more elegant yet, as I don't want let the GUI element call the draw
        // function but rather just return the pixel data.
        // Especially in this case, this deems to be difficult, because the rusttype API forces me
        // to use the glyph.draw() callback, which can only generate all glyph pixels at once.
        // If this wouldn't be the case, I could implement an iterator that computes each pixel on
        // next() ...
        for glyph in &glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let c = (255.0 - 255.0 * v * font_info.saturation) as u8;
                    let x = bounding_box.min.x + x as i32;
                    let y = bounding_box.min.y + y as i32;
                    pixels[(x + y * glyphs_width as i32) as usize] = Color::new(c, c, c);
                });
            }
        }

        Box::new(Text {
            pos: None,
            data: glyphs,
            width: glyphs_width as i16,
            height: glyphs_height as i16,
            font_info,
            pixels,
        })
    }
}

impl<'a> Widget for Text<'a> {
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
