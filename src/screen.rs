use std::cell::RefCell;
use std::rc::Rc;

use crate::framebuffer::{self, Framebuffer, MxcfbRect};
use crate::math::{UVec, Vec2};
use crate::page::Page;
use crate::util::Color;

/* Abstraction over the framebuffer for our eink usecase */
pub struct Screen<'a> {
    pub(crate) fb: Rc<RefCell<Framebuffer>>,
    pub width: u16,
    pub height: u16,
    pages: Rc<RefCell<Vec<Page<'a>>>>,
}

impl<'a> Screen<'a> {
    pub fn new() -> Option<Screen<'a>> {
        let fb = Framebuffer::new("/dev/fb0");
        let fb = match fb {
            Ok(fb) => fb,
            Err(_) => return None,
        };

        let width = fb.var_info.xres as u16;
        let height = fb.var_info.yres as u16;
        let fb = Rc::new(RefCell::new(fb));

        Some(Screen::<'a> {
            fb,
            width,
            height,
            pages: Rc::new(RefCell::new(Vec::new())),
        })
    }

    pub fn add_page(&'a self, page: Page<'a>) {
        let mut vector = self.pages.borrow_mut();
        vector.push(page);
    }

    pub fn render(&self) {
        let pages = self.pages.borrow();
        for p in pages.iter() {
            let _ = p.render();
        }
    }

    pub fn plot(&self, px: UVec, color: Color) -> Result<(), &str> {
        if px.x > self.width || px.y > self.height {
            return Err("pixel outside of screen");
        }

        let mut fb = self.fb.borrow_mut();
        fb.set_pixel(px, [color.r, color.g, color.b]);
        Ok(())
    }

    pub fn clear(&self) {
        let mut fb = self.fb.borrow_mut();
        for x in 0..self.width {
            for y in 0..self.height {
                let _ = fb.set_pixel(Vec2::new(x, y), [255, 255, 255]);
            }
        }
    }
    /* updates the screen with all previous drawing calls.
     * blocks until screen is updated.
     */
    pub fn update(&self) {
        let mut fb = self.fb.borrow_mut();
        let rect = MxcfbRect {
            top: 0,
            left: 0,
            width: self.width as u32,
            height: self.height as u32,
        };

        let mode = framebuffer::Mode::Full;
        match fb.update(rect, mode) {
            Ok(_) => {}
            Err(_) => todo!("Error handling screen update"),
        }

        match fb.wait() {
            Ok(_) => {}
            Err(_) => todo!("Error handling screen wait-for-update"),
        }
    }
}
