pub mod openweathermap;
pub mod weatherapicom;

use chrono::{NaiveDate, Local};
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
    fn get_weather_in_date(weathers: BTreeMap<NaiveDate, f64>, date: Option<NaiveDate>) -> Weather {
        match date {
            Some(d) => {
                Weather {
                    date: d,
                    temperature: weathers.get(&d).unwrap().clone()
                }
            }
            None => {
                Weather {
                    date: Local::now().naive_utc().date(),
                    temperature: weathers.get(&Local::now().naive_utc().date()).unwrap().clone()
                }
            }
        }
    }
}