use crate::gui::gui::Widget;
use crate::math::Vec2;
use crate::screen::Screen;

pub struct Page<'a> {
    pub screen: &'a Screen<'a>,
    pub width: u16,
    pub height: u16,
    pub widgets: Vec<Box<dyn Widget>>,
}

impl<'a> Page<'a> {
    pub fn new(screen: &'a Screen<'a>) -> Page<'a> {
        let width = screen.width;
        let height = screen.height;
        Page {
            screen,
            width,
            height,
            widgets: Vec::new(),
        }
    }
    pub fn add(&mut self, obj: Box<dyn Widget>) {
        self.widgets.push(obj);
    }

    pub fn render(&self) -> Result<(), &str> {
        let screen = self.screen;
        for widget in &self.widgets {
            let data = widget.get_pixel_data();

            let p = widget.get_pos();
            let w = widget.get_width() as usize;
            let h = widget.get_height() as usize;

            for y in 0..h {
                for x in 0..w {
                    let res = screen.plot(p + Vec2::new(x as u16, y as u16), data[y * w + x]);
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
