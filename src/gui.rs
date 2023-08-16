use crate::Region;
use image::DynamicImage;
use rusttype::{Font, PositionedGlyph, Scale};
use std::path::Path;

use crate::util::{Color, FontSetting, Point};
use image::imageops::FilterType;
use image::io::Reader as ImageReader;

// Text, Images, the list goes on...
pub trait Drawable {
    fn draw(&self, region: &Region) -> Result<(), &str>;
}

pub struct Image {
    data: DynamicImage,
    pos: Point,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(filename: &str, pos: Point, scale: f32) -> Box<Image> {
        let file = Path::new(filename);
        let data = ImageReader::open(file).unwrap().decode().unwrap();
        let w: f32 = data.width() as f32;
        let h: f32 = data.height() as f32;

        let data = data.resize((w * scale) as u32, (h * scale) as u32, FilterType::Gaussian);
        let width = data.width();
        let height = data.height();
        Box::new(Image {
            data,
            pos,
            width,
            height,
        })
    }
}

impl Drawable for Image {
    fn draw(&self, region: &Region) -> Result<(), &str> {
        let screen = region.screen.unwrap();
        if region.pos.x + self.pos.x + self.data.width() > screen.width
            || region.pos.y + self.pos.y + self.data.height() > screen.height
        {
            return Err("image outside of frame");
        }
        // turn into grayscale and convert transparent background to white
        let mut img = self.data.to_luma_alpha8();
        for px in img.pixels_mut() {
            px.0[0] = 255 - (px.0[0] * px.0[1]);
        }

        for x in 0..img.width() {
            for y in 0..img.height() {
                let px = img.get_pixel(x, y);
                if let Err(_) = region.plot(
                    self.pos + Point::new(x, y),
                    Color::new(px.0[0], px.0[0], px.0[0]),
                ) {
                    return Err("image outside of frame");
                }
            }
        }
        Ok(())
    }
}

pub struct Text<'a> {
    // text: String
    pub pos: Point,
    pub data: Vec<PositionedGlyph<'a>>,
    pub width: u32,
    pub height: u32,
    pub font_info: FontSetting,
}

impl<'a> Text<'a> {
    pub fn new(text: &str, pos: Point, font_info: FontSetting) -> Box<Text<'a>> {
        let path = std::env::current_dir()
            .unwrap()
            .join("fonts/")
            .join(&font_info.name);
        let size = Scale::uniform(font_info.size);

        let data = std::fs::read(&path).unwrap();

        let font = Font::try_from_vec(data).unwrap();
        let v_metrics = font.v_metrics(size);

        let glyphs: Vec<_> = font
            .layout(text, size, rusttype::point(0.0, v_metrics.ascent))
            .collect();

        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            (max_x - min_x) as u32
        };

        Box::new(Text {
            pos,
            data: glyphs,
            width: glyphs_width,
            height: glyphs_height,
            font_info,
        })
    }
}

impl<'a> Drawable for Text<'a> {
    fn draw(&self, region: &Region) -> Result<(), &str> {
        let screen = region.screen.unwrap();
        if region.pos.x + self.pos.x > screen.width || region.pos.y + self.pos.y > screen.height {
            return Err("text position out of screen bounds");
        }

        if region.pos.x + self.pos.x + self.width > screen.width
            || region.pos.y + self.pos.y + self.height > screen.height
        {
            return Err("text exceeds screen width");
        }

        for glyph in &self.data {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let c = (255.0 - 255.0 * v * self.font_info.saturation) as u8;
                    let _ = region.plot(
                        // we already check if pixel is out of bounds
                        Point::new(
                            self.pos.x + x + bounding_box.min.x as u32,
                            self.pos.y + y + bounding_box.min.y as u32,
                        ),
                        Color::new(c, c, c),
                    );
                });
            }
        }
        Ok(())
    }
}

pub trait BoundingBox {
    fn right_of(&self, margin: u32) -> Point;
    fn below_of(&self, margin: u32) -> Point;
}

impl<'a> BoundingBox for Text<'a> {
    fn right_of(&self, margin: u32) -> Point {
        self.pos + Point::new(self.width + margin, 0)
    }

    fn below_of(&self, margin: u32) -> Point {
        self.pos + Point::new(0, self.height + margin)
    }
}

impl BoundingBox for Image {
    fn right_of(&self, margin: u32) -> Point {
        self.pos + Point::new(self.width + margin, 0)
    }

    fn below_of(&self, margin: u32) -> Point {
        self.pos + Point::new(0, self.height + margin)
    }
}
