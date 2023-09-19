use crate::gui::gui::Widget;
use crate::math::Vec2;
use crate::screen::Screen;

pub struct Page<'a> {
    pub screen: Option<&'a Screen<'a>>,
    pub width: u16,
    pub height: u16,
    pub widgets: Vec<Box<dyn Widget>>,
}

impl<'a> Page<'a> {
    pub fn new() -> Page<'a> {
        Page {
            screen: None,
            width: 0,
            height: 0,
            widgets: Vec::new(),
        }
    }
    pub fn add(&mut self, obj: Box<dyn Widget>) {
        self.widgets.push(obj);
    }

    pub fn render(&self) -> Result<(), &str> {
        let screen = self.screen.unwrap();
        for o in &self.widgets {
            let mut data = o.get_pixel_data().iter();
            let p = o.get_pos();

            let w = o.get_width();
            let h = o.get_height();
            for y in 0..h {
                for x in 0..w {
                    let res = screen.plot(p + Vec2::new(x, y), *data.next().unwrap());
                    match res {
                        Ok(_) => {}
                        Err(_) => panic!("tried to plot pixel outside of screen!"),
                    }
                }
            }
        }
        Ok(())
    }
}
