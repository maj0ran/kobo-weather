use crate::{
    math::{IVec, UVec},
    util::Color,
};

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

pub enum VAlign {
    Up,
    Center,
    Down,
}

pub enum HAlign {
    Left,
    Center,
    Right,
}

pub struct Positioner<'a> {
    pub rel: &'a dyn Widget,
    pub anchor: (HAlign, VAlign),
    pub align: (HAlign, VAlign),
    pub margin: (i16, i16),
}

pub enum Position<'a> {
    Absolute(UVec),
    Relative(&'a Positioner<'a>),
}

/***
 * Trait for GUI-Elements that can be drawn on the E-Ink Display
 ***/
pub trait Widget {
    fn set_pos_abs(mut self, pos: UVec) -> Self
    where
        Self: Sized,
    {
        self.set_pos(pos);
        self
    }
    fn set_pos_rel(mut self, pos: &Positioner) -> Self
    where
        Self: Sized,
    {
        // Position of the Widget we want to position relative to
        let position = IVec::from(
            UVec::from(pos.rel.get_pos()) +
        // Add the position of another anchor of this widget
        match pos.anchor.0 {
            HAlign::Left => UVec::new(0, 0),
            HAlign::Center => UVec::new(pos.rel.get_width()  / 2, 0),
            HAlign::Right => UVec::new(pos.rel.get_width() , 0),
        } + match pos.anchor.1 {
            VAlign::Up => UVec::new(0, 0),
            VAlign::Center => UVec::new(0, pos.rel.get_height()  / 2),
            VAlign::Down => UVec::new(0, pos.rel.get_height() ),
        } -
        // Substract the align of self from the position
        match pos.align.0 {
            HAlign::Left => UVec::new(self.get_width() , 0),
            HAlign::Center => UVec::new(self.get_width() / 2 , 0),
            HAlign::Right => UVec::new(0, 0),
        } - match pos.align.1 {
            VAlign::Up => UVec::new(0, self.get_height()),
            VAlign::Center => UVec::new(0, self.get_height() / 2),
            VAlign::Down => UVec::new(0, 0),
        },
        ) + IVec::new(pos.margin.0, pos.margin.1);

        self.set_pos(UVec::from(position));
        self
    }

    fn get_width(&self) -> u16;
    fn get_height(&self) -> u16;
    fn get_pos(&self) -> UVec;
    fn get_pixel_data(&self) -> Vec<Color>;
    fn set_pos(&mut self, pos: UVec);
}
