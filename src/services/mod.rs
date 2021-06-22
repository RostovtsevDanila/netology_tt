pub mod openweathermap;
pub mod weatherapicom;

use chrono::{DateTime, Local};

pub struct Weather {
    date: DateTime<Local>,
    temperature: f64,
}

impl Weather {
    pub fn date(&self) -> DateTime<Local> {
        self.date
    }

    pub fn temperature(&self) -> f64 {
        self.temperature
    }
}

pub trait WeatherService {
    fn get_weather_today(city: String) -> Result<Weather, ()>;
    fn get_weather_to_special_day(date: DateTime<Local>, city: String) -> Result<Weather, ()>;
    fn get_weather_week_ahead(city: String)-> Result<Vec<Weather>, ()>;
}