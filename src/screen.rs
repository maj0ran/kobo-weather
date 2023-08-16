use std::cell::RefCell;
use std::rc::Rc;

use crate::framebuffer::{self, Framebuffer, MxcfbRect};
use crate::region::Region;
use crate::util::{Color, Point};

/* Abstraction over the framebuffer for our eink usecase */
pub struct Screen<'a> {
    pub(crate) fb: Rc<RefCell<Framebuffer>>,
    pub width: u32,
    pub height: u32,
    regions: Rc<RefCell<Vec<Region<'a>>>>,
}

impl<'a> Screen<'a> {
    pub fn new() -> Option<Screen<'a>> {
        let fb = Framebuffer::new("/dev/fb0");
        let fb = match fb {
            Ok(fb) => fb,
            Err(_) => return None,
        };

        let width = fb.var_info.xres;
        let height = fb.var_info.yres;
        let fb = Rc::new(RefCell::new(fb));

        Some(Screen::<'a> {
            fb,
            width,
            height,
            regions: Rc::new(RefCell::new(Vec::new())),
        })
    }

    pub fn add_region(&'a self, mut region: Region<'a>) {
        region.screen = Some(&self);
        let mut vector = self.regions.borrow_mut();
        vector.push(region);
    }

    pub fn render(&self) {
        let regions = self.regions.borrow();
        for r in regions.iter() {
            let _ = r.render();
        }
    }

    pub fn plot(&self, px: Point, color: Color) -> Result<(), &str> {
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
                let _ = fb.set_pixel(Point::new(x, y), [255, 255, 255]);
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
            width: self.width,
            height: self.height,
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
