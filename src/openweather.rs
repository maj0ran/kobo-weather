use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    pub country: String,
    pub sunrise: u32,
    pub sunset: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Rain {
    pub one_hour: f32,
    pub three_hour: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snow {
    pub one_hour: f32,
    pub three_hour: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    pub all: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: u16,
    pub humidity: u16,
    pub sea_level: u16,
    pub grnd_level: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wind {
    pub speed: f32,
    pub deg: f32,
    pub gust: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenWeather {
    pub name: String,
    pub coord: Coord,
    pub timezone: u16,
    pub weather: Vec<Weather>,
    pub wind: Wind,
    pub main: Main,
    pub clouds: Clouds,
    pub visibility: i32,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub dt: i64,
    pub sys: Sys,
}

pub fn get_weather() -> std::io::Result<OpenWeather> {
    println!("Hello, world!");

    let mut stream = TcpStream::connect("api.openweathermap.org:80")?;

    stream.write("GET /data/2.5/weather?appid=0cface643ba4c958e2b8174fbbb7170a&lat=52.1508&lon=9.9511&lang=de&units=metric&mode=json HTTP/1.0\r\n\r\n".as_bytes())?;
    let mut buf: [u8; 4096] = [' ' as u8; 4096];
    stream.read(&mut buf)?;

    let response = str::from_utf8(&buf).unwrap();
    let response: Option<(&str, &str)> = response.split_once("\r\n\r\n");
    let response = response.unwrap().1.trim();

    let response: OpenWeather = serde_json::from_str(&response)?;

    Ok(response)
}
