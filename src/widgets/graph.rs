use crate::{
    math::UVec,
    util::{Color, Line, Rectangle},
};

use super::widget::{Position, Widget};

pub struct Graph {
    pos: UVec,
    width: u16,
    height: u16,
    min: i16,
    max: i16,
    step_count: u16,
}

impl Graph {
    pub fn new(
        width: u16,
        height: u16,
        min: i16,
        max: i16,
        step_count: u16,
        pos: Position,
    ) -> Box<Graph> {
        let w = Graph {
            pos: UVec { x: 0, y: 0 },
            width,
            height,
            min,
            max,
            step_count,
        };

        let w = match pos {
            Position::Absolute(p) => w.set_pos_abs(p),
            Position::Relative(p) => w.set_pos_rel(p),
        };

        Box::new(w)
    }
}

impl Widget for Graph {
    widget!();

    fn make(&self) -> Vec<Color> {
        const AXIS_THICKNESS: u16 = 10;

        let mut pixels =
            vec![Color::new(255, 255, 255); self.width as usize * self.height as usize];

        let x_axis = Rectangle::new(
            UVec::new(0, self.height - AXIS_THICKNESS),
            self.width,
            AXIS_THICKNESS,
        );
        let y_axis = Rectangle::new(UVec::new(0, 0), AXIS_THICKNESS, self.height);

        for p in x_axis {
            pixels[p.y as usize * self.width as usize + p.x as usize] = Color::new(0, 0, 0)
        }
        for p in y_axis {
            pixels[p.y as usize * self.width as usize + p.x as usize] = Color::new(0, 0, 0)
        }

        pixels
    }
}
