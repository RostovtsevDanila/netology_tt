pub mod openweathermap;
pub mod weatherapicom;

use chrono::{NaiveDate};
use std::collections::BTreeMap;


pub struct Weather {
    date: NaiveDate,
    temperature: f64,
}

impl Weather {
    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn temperature(&self) -> f64 {
        self.temperature
    }
}


trait WeatherService {
    fn get_weather(city: String, s_key: String) -> Result<BTreeMap<NaiveDate, f64>, ()>;
}