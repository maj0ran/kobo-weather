use crate::{
    math::{IVec, UVec},
    util::{Color, Line},
};

use super::gui::Widget;

pub struct Graph {
    pos: UVec,
    width: u16,
    height: u16,
    min: i16,
    max: i16,
    step_count: u16,
    pixels: Vec<Color>,
}

impl Graph {
    pub fn new(width: u16, height: u16, min: i16, max: i16, step_count: u16) -> Box<Graph> {
        let x_axis = Line::new(IVec::new(0, 0), IVec::new(width as i16, 0), 8);
        let y_axis = Line::new(IVec::new(4, 0), IVec::new(4, height as i16), 8);
        let l = Line::new(IVec::new(200, 200), IVec::new(1200, 1200), 6);
        let mut pixels = vec![Color::new(255, 255, 255); width as usize * height as usize];

        for p in l {
            pixels[p.y as usize * width as usize + p.x as usize] = Color::new(0, 0, 0)
        }

        Box::new(Graph {
            pos: UVec { x: 0, y: 0 },
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
        let pixels = Vec::<Color>::new();
        pixels
    }

    fn set_pos(&mut self, pos: UVec) {
        self.pos = pos
    }
}
