use crate::{math::Vec2, util::Color};

/***
 * Alignment rules for relative positioning
 ***/
#[allow(unused)]
pub enum Align {
    Left,
    Right,
    RightCenter,
    BelowLeft,
    BelowRight,
    BelowCenter,
    AboveLeft,
    AboveRight,
    AboveCenter,
}

/***
 * Trait for GUI-Elements that can be drawn on the E-Ink Display
 ***/
pub trait Widget {
    fn get_width(&self) -> i16;
    fn get_height(&self) -> i16;
    fn get_pos(&self) -> Option<Vec2<i16>>;
    fn get_pixel_data(&self) -> &Vec<Color>;
    fn set_pos(&mut self, pos: Vec2<i16>);
    fn set_pos_rel(&mut self, other: &dyn Widget, align: Align, margin: i16) {
        let p = match align {
            Align::Left => {
                if let Some(p) = other.get_pos() {
                    Some(p - Vec2::new(other.get_width() + margin, 0))
                } else {
                    None
                }
            }
            Align::Right => {
                if let Some(p) = other.get_pos() {
                    Some(p + Vec2::new(other.get_width() + margin, 0))
                } else {
                    None
                }
            }
            Align::BelowLeft => {
                if let Some(p) = other.get_pos() {
                    Some(p + Vec2::new(0, other.get_height() + margin))
                } else {
                    None
                }
            }
            Align::BelowRight => {
                if let Some(p) = other.get_pos() {
                    Some(
                        p + Vec2::new(
                            other.get_width() - self.get_width(),
                            other.get_height() + margin,
                        ),
                    )
                } else {
                    None
                }
            }
            Align::RightCenter => {
                if let Some(p) = other.get_pos() {
                    Some(
                        p + Vec2::new(
                            other.get_width() + margin,
                            other.get_height() / 2 - self.get_height() / 2,
                        ),
                    )
                } else {
                    None
                }
            }
            Align::BelowCenter => {
                if let Some(p) = other.get_pos() {
                    Some(
                        p + Vec2::new(
                            other.get_width() / 2 - self.get_width() / 2,
                            other.get_height() + margin,
                        ),
                    )
                } else {
                    None
                }
            }
            Align::AboveLeft => {
                if let Some(p) = other.get_pos() {
                    Some(p + Vec2::new(0, -(self.get_height() + margin)))
                } else {
                    None
                }
            }
            Align::AboveRight => {
                if let Some(p) = other.get_pos() {
                    Some(p + Vec2::new(other.get_width(), -(self.get_height() + margin)))
                } else {
                    None
                }
            }
            Align::AboveCenter => {
                if let Some(p) = other.get_pos() {
                    Some(
                        p + Vec2::new(
                            other.get_width() / 2 - self.get_width() / 2,
                            -(self.get_height() + margin),
                        ),
                    )
                } else {
                    None
                }
            }
        };

        self.set_pos(p.unwrap());
    }
}
