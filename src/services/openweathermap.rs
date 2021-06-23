use crate::services::{WeatherService, Weather};
use chrono::{Local, DateTime};
use std::collections::HashMap;

pub struct OpenWeatherMap {}

impl WeatherService for OpenWeatherMap {
    fn get_weather_current(city: String, s_key: String) -> Result<Weather, ()> {
        #[derive(Deserialize, Serialize, Debug, Clone)]
        struct OpenWeatherMapResponse {
            main: OpenWeatherMapMain,
        }

        #[derive(Deserialize, Serialize, Debug, Clone)]
        struct OpenWeatherMapMain {
            temp: f64
        }

        let mut query_params = HashMap::new();
        query_params.insert("q", city);
        query_params.insert("appid", s_key);

        let res = reqwest::blocking::Client::new().get("http://api.openweathermap.org/data/2.5/weather")
            .query(&query_params)
            .send()
            .unwrap()
            .json::<OpenWeatherMapResponse>()
            .unwrap();

        Ok(
            Weather {
                date: Local::now(),
                temperature: res.main.temp - 273.15
            }
        )
    }

    fn get_weather_to_special_day(date: DateTime<Local>, city: String, s_key: String) -> Result<Weather, ()> {
        todo!()
    }

    fn get_weather_week_ahead(city: String, s_key: String) -> Result<Vec<Weather>, ()> {
        todo!()
    }
}

