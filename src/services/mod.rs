pub mod openweathermap;
pub mod weatherapicom;

use chrono::{DateTime, Local, NaiveDate};
use std::collections::BTreeMap;


pub struct WeatherCurrent {
    datetime: DateTime<Local>,
    temperature: f64,
}

impl WeatherCurrent {
    #[allow(dead_code)]
    pub fn date(&self) -> DateTime<Local> {
        self.datetime
    }

    pub fn temperature(&self) -> f64 {
        self.temperature
    }
}


pub trait WeatherService {
    fn get_weather_current(city: String, s_key: String) -> Result<WeatherCurrent, ()>;
    fn get_weather_week_ahead(city: String, s_key: String)-> Result<BTreeMap<NaiveDate, f64>, ()>;
}