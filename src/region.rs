use crate::gui::{BoundingBox, Drawable};
use crate::screen::Screen;
use crate::util::{Color, Point};

pub struct Region<'a> {
    pub screen: Option<&'a Screen<'a>>,
    pub pos: Point,
    pub width: u32,
    pub height: u32,
    pub border: bool,
    pub objects: Vec<Box<dyn Drawable>>,
}

impl<'a> Region<'a> {
    pub fn new(pos: Point, width: u32, height: u32, border: bool) -> Region<'a> {
        Region {
            screen: None,
            pos,
            width,
            height,
            border,
            objects: Vec::new(),
        }
    }
    pub fn add_object(&mut self, obj: Box<dyn Drawable>) {
        self.objects.push(obj);
    }

    pub fn render(&self) -> Result<(), &str> {
        if self.border {
            // no error checking because we already know from region creation that it's not out of
            // bounds
            for x in 0..self.width {
                let _ = self.plot(Point::new(x, 0), Color::new(0, 0, 0));
                let _ = self.plot(Point::new(x, self.height), Color::new(0, 0, 0));
            }

            for y in 0..self.height {
                let _ = self.plot(Point::new(0, y), Color::new(0, 0, 0));
                let _ = self.plot(Point::new(self.width, y), Color::new(0, 0, 0));
            }
        }

        for o in &self.objects {
            match o.draw(self) {
                Ok(_) => {}
                Err(_) => return Err("render call out of screen"),
            }
        }
        Ok(())
    }

    pub fn plot(&self, px: Point, color: Color) -> Result<(), &str> {
        let screen = self.screen.unwrap();
        screen.plot(self.pos + px, color)
    }
}

impl<'a> BoundingBox for Region<'a> {
    fn right_of(&self, margin: u32) -> Point {
        self.pos + Point::new(self.width + margin, 0)
    }

    fn below_of(&self, margin: u32) -> Point {
        self.pos + Point::new(0, self.height + margin)
    }
}
