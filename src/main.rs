mod framebuffer;
mod gui;
mod math;
mod openweather;
mod page;
mod screen;
mod util;
mod weather_data;

use gui::graph::Graph;
use gui::gui::{Align, Widget};

use gui::image::Image;
use gui::text::Text;
use openweather::get_weather;
use page::Page;
use screen::Screen;
use util::FontSetting;
use weather_data::WeatherData;

use crate::math::Vec2;

#[allow(unused)]
const FONT_BIG: FontSetting = FontSetting {
    name: "LucidaTypewriterRegular.ttf",
    size: 160.0,
    saturation: 1.0,
};
#[allow(unused)]
const FONT_BIGMED: FontSetting = FontSetting {
    name: "LucidaTypewriterRegular.ttf",
    size: 128.0,
    saturation: 1.0,
};
#[allow(unused)]
const FONT_MED: FontSetting = FontSetting {
    name: "LucidaTypewriterRegular.ttf",
    size: 96.0,
    saturation: 1.0,
};
#[allow(unused)]
const FONT_SMALLMED: FontSetting = FontSetting {
    name: "LucidaTypewriterRegular.ttf",
    size: 64.0,
    saturation: 1.0,
};
#[allow(unused)]
const FONT_SMALL: FontSetting = FontSetting {
    name: "LucidaTypewriterRegular.ttf",
    size: 48.0,
    saturation: 1.0,
};

#[allow(unused)]
const FONT_TINY: FontSetting = FontSetting {
    name: "LucidaTypewriterRegular.ttf",
    size: 32.0,
    saturation: 1.0,
};

fn main() -> std::io::Result<()> {
    let screen = Screen::new().unwrap(); // just panic, this program is without screen useless
    println!("Screen Dimension: {}x{}", screen.width, screen.height);

    /*** Page ***/
    let mut mainpage = Page::new();
    /*** Fetch Weather Data ***/
    let openweather = get_weather().unwrap();
    let weather = WeatherData::new(openweather);

    /*** create GUI objects ***/

    /*** date, time, location ***/
    let mut locale = Text::new(&weather.locale, FONT_TINY);
    locale.set_pos(Vec2::new(screen.width / 2 - locale.width / 2, 16));

    /*** Temperature ***/
    let mut temp = Text::new(&weather.temp, FONT_BIG);
    temp.set_pos(Vec2::new(64, 64));

    let mut temp_icon = Image::new("icons/C.png", 2.0);
    temp_icon.set_pos_rel(temp.as_ref(), Align::Right, 5);

    let mut temp_min = Text::new(&weather.temp_min, FONT_SMALLMED);
    temp_min.set_pos_rel(temp.as_ref(), Align::BelowLeft, 10);

    let mut temp_max = Text::new(&weather.temp_max, FONT_SMALLMED);
    temp_max.set_pos_rel(temp.as_ref(), Align::BelowRight, 10);

    let mut temp_feels = Text::new(&weather.temp_feels, FONT_SMALLMED);
    temp_feels.set_pos_rel(temp.as_ref(), Align::RightCenter, 50);

    /*** Sky ***/
    let mut weather_icon = Image::new(&("icons/" + weather.icon + ".png"), 6.0);
    weather_icon.set_pos(Vec2::new(screen.width / 2 + 16, 96));

    let mut suntime = Text::new(&(weather.sunrise + " - " + &weather.sunset), FONT_SMALL);
    suntime.set_pos_rel(weather_icon.as_ref(), Align::AboveCenter, 0);

    let mut desc = Text::new(&weather.desc, FONT_MED);
    desc.set_pos_rel(weather_icon.as_ref(), Align::BelowCenter, 0);

    let mut cloudiness = Text::new(&(weather.cloudiness + "% bewölkt"), FONT_SMALLMED);
    cloudiness.set_pos_rel(desc.as_ref(), Align::BelowCenter, 16);

    /*** Atmosphere ***/
    let mut humidity = Text::new(&("φ: " + weather.humidity + " %"), FONT_SMALLMED);
    humidity.set_pos_rel(temp_min.as_ref(), Align::BelowLeft, 100);

    let mut pressure_grnd = Text::new(&("ρ: " + weather.grnd_level + " hPa"), FONT_SMALLMED);
    pressure_grnd.set_pos_rel(humidity.as_ref(), Align::BelowLeft, 10);

    let mut pressure_sea = Text::new(&("ϱ: " + weather.sea_level + " hPa"), FONT_SMALLMED);
    pressure_sea.set_pos_rel(pressure_grnd.as_ref(), Align::BelowLeft, 10);

    /*** Wind ***/
    let mut w_icon = Image::new("icons/w.png", 2.0);
    w_icon.set_pos(Vec2::new(16, screen.height - 128));

    let mut w_speed = Text::new(&(weather.wind_speed + "m/s"), FONT_MED);
    w_speed.set_pos_rel(w_icon.as_ref(), Align::RightCenter, 50);

    let mut w_gust = Text::new(&("(" + weather.wind_gust + "m/s)"), FONT_MED);
    w_gust.set_pos_rel(w_speed.as_ref(), Align::Right, 30);

    let mut w_dir = Text::new(&(weather.wind_dir + "°"), FONT_MED);
    w_dir.set_pos_rel(w_gust.as_ref(), Align::Right, 50);

    /*** Rain ***/
    let mut r_icon = Image::new("icons/h.png", 2.0);
    r_icon.set_pos_rel(w_icon.as_ref(), Align::AboveLeft, 50);

    let mut r_1h = Text::new(&("1h: " + weather.rain_1h + "mm"), FONT_MED);
    r_1h.set_pos_rel(r_icon.as_ref(), Align::Right, 80);

    let mut r_3h = Text::new(&("3h: " + weather.rain_3h + "mm"), FONT_MED);
    r_3h.set_pos_rel(r_1h.as_ref(), Align::Right, 100);

    /*** Snow ***/
    let mut s_icon = Image::new("icons/13d.png", 2.0);
    s_icon.set_pos_rel(r_icon.as_ref(), Align::AboveLeft, 50);

    let mut s_1h = Text::new(&("1h: " + weather.snow_1h + "mm"), FONT_MED);
    s_1h.set_pos_rel(s_icon.as_ref(), Align::Right, 30);

    let mut s_3h = Text::new(&("3h: " + weather.snow_3h + "mm"), FONT_MED);
    s_3h.set_pos_rel(s_1h.as_ref(), Align::Right, 100);

    let mut g = Graph::new(screen.width, screen.height, 0, 30, 5);
    g.set_pos(Vec2::new(0, 0));

    /*** add GUI objects to pages ***/
    mainpage.add(locale);
    mainpage.add(temp);
    mainpage.add(temp_min);
    mainpage.add(temp_max);
    mainpage.add(temp_feels);
    mainpage.add(temp_icon);
    mainpage.add(weather_icon);
    mainpage.add(desc);
    mainpage.add(cloudiness);
    mainpage.add(suntime);
    mainpage.add(w_icon);
    mainpage.add(w_speed);
    mainpage.add(w_dir);
    mainpage.add(w_gust);
    mainpage.add(r_icon);
    mainpage.add(r_1h);
    mainpage.add(r_3h);
    mainpage.add(s_icon);
    mainpage.add(s_1h);
    mainpage.add(s_3h);

    mainpage.add(humidity);
    mainpage.add(pressure_grnd);
    mainpage.add(pressure_sea);

    mainpage.add(g);
    /*** add pages to screen ***/
    screen.add_page(mainpage);
    /*** draw ***/
    screen.clear();
    screen.render();
    screen.update();

    Ok(())
}
