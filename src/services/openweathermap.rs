use crate::services::{WeatherService, Weather};
use chrono::{Local, DateTime};

pub struct OpenWeatherMap {
    api_key: String
}

impl WeatherService for OpenWeatherMap {
    fn get_weather_today(city: String) -> Result<Weather, ()> {
        todo!()
    }

    fn get_weather_to_special_day(date: DateTime<Local>, city: String) -> Result<Weather, ()> {
        todo!()
    }

    fn get_weather_week_ahead(city: String) -> Result<Vec<Weather>, ()> {
        todo!()
    }
}