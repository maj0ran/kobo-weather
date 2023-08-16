mod framebuffer;
mod gui;
mod openweather;
mod region;
mod screen;
mod util;

use gui::BoundingBox;

use gui::{Image, Text};
use openweather::get_weather;
use region::Region;
use screen::Screen;
use util::{FontSetting, Point};

#[allow(unused)]
const FONT_BIG: FontSetting = FontSetting {
    name: "LucidaTypewriterRegular.ttf",
    size: 192.0,
    saturation: 1.0,
};

#[allow(unused)]
const FONT_SMALL: FontSetting = FontSetting {
    name: "LucidaTypewriterRegular.ttf",
    size: 32.0,
    saturation: 1.0,
};

#[allow(unused)]
const FONT_MED: FontSetting = FontSetting {
    name: "LucidaTypewriterRegular.ttf",
    size: 96.0,
    saturation: 1.0,
};

const MARGIN: u32 = 10;
/* when we encounter an error, call this function. it will display a notification on the screen,
 * then exits the program. due to the nature of eink displays, the notification will stay on the
 * screen.
 */
//fn error_state(screen: &Screen) {
//    screen.clear();
//
//    let mut region = Region::new(
//        screen,
//        Point::new(0, 0),
//        screen.width - 1,
//        screen.height - 1,
//        false,
//    );
//
//    let error_text = Text::new(
//        "Error encountered! Stop.",
//        Point::new(250, screen.height / 2),
//        font_big,
//    );
//    region.add_object(error_text);
//    let _ = region.render(); // tough luck if this errors too
//    screen.update();
//
//    panic!("Stopping after error");
//}

fn main() -> std::io::Result<()> {
    let screen = Screen::new().unwrap(); // just panic, this program is without screen useless

    /*** Regions ***/
    let mut topbar = Region::new(
        Point::new(MARGIN, MARGIN),
        screen.width - MARGIN * 2,
        50,
        true,
    );
    let mut today = Region::new(
        topbar.below_of(10),
        screen.width / 2 - MARGIN,
        screen.height / 2 - MARGIN * 2,
        true,
    );
    /*** Fetch Weather Data ***/
    let weather = get_weather().unwrap();

    let temp = weather.main.temp;
    let temp = format!("{:>2.1}", temp);
    let country = weather.sys.country;
    let city = weather.name;
    let dt = weather.dt;
    let datetime = chrono::NaiveDateTime::from_timestamp(dt, 0);
    let datetime = datetime.format("%a, %d. %B %Y %H:%m:%S");
    let localization = city + ", " + &country + ", " + &datetime.to_string();

    /*** create GUI objects ***/
    let localization = Text::new(&localization, Point::new(MARGIN, MARGIN), FONT_SMALL);
    let temp = Text::new(&temp, Point::new(MARGIN, MARGIN), FONT_BIG);
    let temp_icon = Image::new("icons/C.png", temp.right_of(10), 2.0);

    /*** add GUI objects to regions ***/
    topbar.add_object(localization);
    today.add_object(temp);
    today.add_object(temp_icon);

    /*** add Regions to screen ***/
    screen.add_region(topbar);
    screen.add_region(today);
    /*** draw ***/
    screen.clear();
    screen.render();
    screen.update();

    Ok(())
}
