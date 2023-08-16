use crate::{
    math::Vec2,
    util::{Color, Line},
};

use super::gui::Widget;

pub struct Graph {
    pos: Option<Vec2<i16>>,
    width: i16,
    height: i16,
    min: i16,
    max: i16,
    step_count: u16,
    pixels: Vec<Color>,
}

impl Graph {
    pub fn new(width: i16, height: i16, min: i16, max: i16, step_count: u16) -> Box<Graph> {
        let x_axis = Line::new(Vec2::new(0, 0), Vec2::new(width, 0), 8);
        let y_axis = Line::new(Vec2::new(4, 0), Vec2::new(4, height), 8);
        let l = Line::new(Vec2::<i16>::new(200, 200), Vec2::<i16>::new(1200, 1200), 6);
        let mut pixels = vec![Color::new(255, 255, 255); width as usize * height as usize];

        for p in l {
            pixels[p.y as usize * width as usize + p.x as usize] = Color::new(0, 0, 0)
        }

        Box::new(Graph {
            pos: None,
            width,
            height,
            min,
            max,
            step_count,
            pixels,
        })
    }
}

impl Widget for Graph {
    fn get_width(&self) -> i16 {
        self.width
    }

    fn get_height(&self) -> i16 {
        self.height
    }

    fn get_pos(&self) -> Option<Vec2<i16>> {
        self.pos
    }

    fn get_pixel_data(&self) -> &Vec<crate::util::Color> {
        &self.pixels
    }

    fn set_pos(&mut self, pos: Vec2<i16>) {
        self.pos = Some(pos)
    }
}
