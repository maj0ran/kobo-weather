mod framebuffer;
mod gui;
mod math;
mod openweather;
mod page;
mod screen;
mod util;
mod weather_data;

use gui::gui::{HAlign, VAlign};

use gui::image::Image;
use gui::text::Text;
use openweather::get_weather;
use page::Page;
use screen::Screen;
use util::FontSetting;
use weather_data::WeatherData;

use crate::gui::gui::{Position, Positioner};
use crate::math::{UVec, Vec2};

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
    let mut mainpage = Page::new(&screen);
    /*** Fetch Weather Data ***/
    let openweather = get_weather().unwrap();
    let weather = WeatherData::new(openweather);

    /*** create GUI objects ***/

    /*** date, time, location ***/
    let locale = Text::new(
        &weather.locale,
        FONT_TINY,
        Position::Absolute(UVec::new(0, 0)),
    );
    /*** Temperature ***/
    let temp = Text::new(
        &weather.temp,
        FONT_BIG,
        Position::Absolute(UVec::new(64, 64)),
    );

    let temp_icon = Image::new(
        "icons/C.png",
        2.0,
        Position::Relative(&Positioner {
            rel: temp.as_ref(),
            anchor: (HAlign::Right, VAlign::Up),
            align: (HAlign::Right, VAlign::Down),
            margin: (0, 0),
        }),
    );

    let temp_min = Text::new(
        &weather.temp_min,
        FONT_SMALLMED,
        Position::Relative(&Positioner {
            rel: temp.as_ref(),
            anchor: (HAlign::Left, VAlign::Down),
            align: (HAlign::Right, VAlign::Down),
            margin: (0, 0),
        }),
    );

    let temp_max = Text::new(
        &weather.temp_max,
        FONT_SMALLMED,
        Position::Relative(&Positioner {
            rel: temp.as_ref(),
            anchor: (HAlign::Right, VAlign::Down),
            align: (HAlign::Left, VAlign::Down),
            margin: (0, 0),
        }),
    );

    let temp_feels = Text::new(
        &weather.temp_feels,
        FONT_SMALLMED,
        Position::Relative(&Positioner {
            rel: temp.as_ref(),
            anchor: (HAlign::Right, VAlign::Center),
            align: (HAlign::Right, VAlign::Center),
            margin: (32, 0),
        }),
    );

    /*** Sky ***/
    let weather_icon = Image::new(
        &("icons/" + weather.icon + ".png"),
        6.0,
        Position::Absolute(UVec::new(screen.width / 2 + 16, 96)),
    );

    let suntime = Text::new(
        &(weather.sunrise + " - " + &weather.sunset),
        FONT_SMALL,
        Position::Relative(&Positioner {
            rel: weather_icon.as_ref(),
            anchor: (HAlign::Center, VAlign::Up),
            align: (HAlign::Center, VAlign::Up),
            margin: (0, 0),
        }),
    );

    let desc = Text::new(
        &weather.desc,
        FONT_MED,
        Position::Relative(&Positioner {
            rel: weather_icon.as_ref(),
            anchor: (HAlign::Center, VAlign::Down),
            align: (HAlign::Center, VAlign::Down),
            margin: (0, 0),
        }),
    );

    let cloudiness = Text::new(
        &(weather.cloudiness + "% bewölkt"),
        FONT_SMALLMED,
        Position::Relative(&Positioner {
            rel: desc.as_ref(),
            anchor: (HAlign::Center, VAlign::Down),
            align: (HAlign::Center, VAlign::Down),
            margin: (0, 0),
        }),
    );

    /*** Atmosphere ***/
    let humidity = Text::new(
        &("φ:   " + weather.humidity + " %"),
        FONT_SMALLMED,
        Position::Relative(&Positioner {
            rel: temp_min.as_ref(),
            anchor: (HAlign::Left, VAlign::Down),
            align: (HAlign::Right, VAlign::Down),
            margin: (0, 106),
        }),
    );

    let pressure_grnd = Text::new(
        &("ρ: " + weather.grnd_level + " hPa"),
        FONT_SMALLMED,
        Position::Relative(&Positioner {
            rel: humidity.as_ref(),
            anchor: (HAlign::Left, VAlign::Down),
            align: (HAlign::Right, VAlign::Down),
            margin: (0, 0),
        }),
    );

    let pressure_sea = Text::new(
        &("ϱ: " + weather.sea_level + " hPa"),
        FONT_SMALLMED,
        Position::Relative(&Positioner {
            rel: pressure_grnd.as_ref(),
            anchor: (HAlign::Left, VAlign::Down),
            align: (HAlign::Right, VAlign::Down),
            margin: (0, 0),
        }),
    );

    /*** Wind ***/
    let w_icon = Image::new(
        "icons/w.png",
        2.0,
        Position::Absolute(Vec2::new(16, screen.height - 96)),
    );

    let w_speed = Text::new(
        &(weather.wind_speed + "m/s"),
        FONT_MED,
        Position::Relative(&Positioner {
            rel: w_icon.as_ref(),
            anchor: (HAlign::Right, VAlign::Center),
            align: (HAlign::Right, VAlign::Center),
            margin: (16, 0),
        }),
    );
    let w_gust = Text::new(
        &("(" + weather.wind_gust + "m/s)"),
        FONT_MED,
        Position::Relative(&Positioner {
            rel: w_speed.as_ref(),
            anchor: (HAlign::Right, VAlign::Center),
            align: (HAlign::Right, VAlign::Center),
            margin: (16, 0),
        }),
    );

    let w_dir = Text::new(
        &(weather.wind_dir + "°"),
        FONT_MED,
        Position::Relative(&Positioner {
            rel: w_gust.as_ref(),
            anchor: (HAlign::Right, VAlign::Center),
            align: (HAlign::Right, VAlign::Center),
            margin: (16, 0),
        }),
    );

    /*** Rain ***/
    let r_icon = Image::new(
        "icons/h.png",
        2.0,
        Position::Relative(&Positioner {
            rel: w_icon.as_ref(),
            anchor: (HAlign::Left, VAlign::Up),
            align: (HAlign::Right, VAlign::Up),
            margin: (0, -48),
        }),
    );

    let r_1h = Text::new(
        &("1h: " + weather.rain_1h + "mm"),
        FONT_MED,
        Position::Relative(&Positioner {
            rel: w_speed.as_ref(),
            anchor: (HAlign::Left, VAlign::Up),
            align: (HAlign::Right, VAlign::Up),
            margin: (0, -48),
        }),
    );

    let r_3h = Text::new(
        &("3h: " + weather.rain_3h + "mm"),
        FONT_MED,
        Position::Relative(&Positioner {
            rel: r_1h.as_ref(),
            anchor: (HAlign::Right, VAlign::Center),
            align: (HAlign::Right, VAlign::Center),
            margin: (64, 0),
        }),
    );

    /*** Snow ***/
    let s_icon = Image::new(
        "icons/13d.png",
        2.0,
        Position::Relative(&Positioner {
            rel: r_icon.as_ref(),
            anchor: (HAlign::Left, VAlign::Up),
            align: (HAlign::Right, VAlign::Up),
            margin: (0, -48),
        }),
    );

    let s_1h = Text::new(
        &("1h: " + weather.snow_1h + "mm"),
        FONT_MED,
        Position::Relative(&Positioner {
            rel: r_1h.as_ref(),
            anchor: (HAlign::Left, VAlign::Up),
            align: (HAlign::Right, VAlign::Up),
            margin: (0, -48),
        }),
    );

    let s_3h = Text::new(
        &("3h: " + weather.snow_3h + "mm"),
        FONT_MED,
        Position::Relative(&Positioner {
            rel: s_1h.as_ref(),
            anchor: (HAlign::Right, VAlign::Center),
            align: (HAlign::Right, VAlign::Center),
            margin: (64, 0),
        }),
    );

    //   let mut g = Graph::new(screen.width, screen.height, 0, 30, 5);
    //   g.set_pos(Vec2::new(0, 0));

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
    //
    mainpage.add(humidity);
    mainpage.add(pressure_grnd);
    mainpage.add(pressure_sea);

    //  mainpage.add(g);
    /*** add pages to screen ***/
    screen.add_page(mainpage);
    /*** draw ***/
    screen.clear();
    screen.render();
    screen.update();

    Ok(())
}
