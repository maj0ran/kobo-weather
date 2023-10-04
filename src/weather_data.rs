use std::ops::{self, Deref};

use crate::openweather::OpenWeather;

// wrapper around string so we can implement into() / from() for various data types, as our E-Ink
// Textfields only accept Strings
#[derive(Debug)]
pub struct InkString(String);

impl ops::Add<&str> for InkString {
    type Output = InkString;

    fn add(self, rhs: &str) -> Self::Output {
        InkString { 0: self.0 + rhs }
    }
}

impl ops::Add<InkString> for &str {
    type Output = InkString;

    fn add(self, rhs: InkString) -> Self::Output {
        InkString {
            0: self.to_string() + rhs.0.as_str(),
        }
    }
}

impl From<u32> for InkString {
    fn from(value: u32) -> Self {
        InkString {
            0: format!("{}", value),
        }
    }
}
impl From<u16> for InkString {
    fn from(value: u16) -> Self {
        InkString {
            0: format!("{}", value),
        }
    }
}
impl From<f32> for InkString {
    fn from(value: f32) -> Self {
        InkString {
            0: format!("{:>2.1}", value),
        }
    }
}

impl From<String> for InkString {
    fn from(value: String) -> Self {
        InkString { 0: value }
    }
}

impl Deref for InkString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct WeatherData {
    pub icon: InkString,
    pub temp: InkString,
    pub temp_min: InkString,
    pub temp_max: InkString,
    pub locale: InkString,
    pub pressure: InkString,
    pub grnd_level: InkString,
    pub sea_level: InkString,
    pub humidity: InkString,
    pub desc: InkString,
    pub temp_feels: InkString,
    pub wind_speed: InkString,
    pub wind_dir: InkString,
    pub wind_gust: InkString,
    pub cloudiness: InkString,
    pub rain_1h: InkString,
    pub rain_3h: InkString,
    pub snow_1h: InkString,
    pub snow_3h: InkString,
    pub sunrise: InkString,
    pub sunset: InkString,
}

impl WeatherData {
    pub fn new(data: OpenWeather) -> WeatherData {
        WeatherData {
            icon: data.weather[0].icon.clone().into(),
            temp: data.main.temp.into(),
            temp_min: data.main.temp_min.into(),
            temp_max: data.main.temp_max.into(),
            locale: {
                let country = data.sys.country;
                let city = data.name;
                let dt = data.dt;
                let datetime = chrono::NaiveDateTime::from_timestamp(dt, 0);
                let datetime = datetime.format("%a, %d. %B %Y %H:%m:%S");
                (city + ", " + &country + ", " + &datetime.to_string()).into()
            },
            pressure: data.main.pressure.into(),
            humidity: data.main.humidity.into(),
            sea_level: format!("{:>4}", data.main.sea_level).into(),
            grnd_level: format!("{:>4}", data.main.grnd_level).into(),
            desc: data.weather[0].description.clone().into(),
            temp_feels: data.main.feels_like.into(),
            wind_speed: data.wind.speed.into(),
            wind_dir: data.wind.deg.into(),
            wind_gust: data.wind.gust.into(),
            cloudiness: data.clouds.all.into(),
            rain_1h: {
                if let Some(r) = &data.rain {
                    if let Some(r) = r.one_hour {
                        r.into()
                    } else {
                        0.0.into()
                    }
                } else {
                    0.0.into()
                }
            },
            rain_3h: {
                if let Some(r) = &data.rain {
                    if let Some(r) = r.three_hour {
                        r.into()
                    } else {
                        0.0.into()
                    }
                } else {
                    0.0.into()
                }
            },
            snow_1h: {
                if let Some(s) = &data.snow {
                    s.one_hour.into()
                } else {
                    0.0.into()
                }
            },
            snow_3h: {
                if let Some(s) = &data.snow {
                    s.three_hour.into()
                } else {
                    0.0.into()
                }
            },
            sunrise: {
                let ts = data.sys.sunrise.into();
                let datetime = chrono::NaiveDateTime::from_timestamp(ts, 0);
                datetime.format("%H:%m").to_string().into()
            },
            sunset: {
                let ts = data.sys.sunset.into();
                let datetime = chrono::NaiveDateTime::from_timestamp(ts, 0);
                datetime.format("%H:%m").to_string().into()
            },
        }
    }
}
