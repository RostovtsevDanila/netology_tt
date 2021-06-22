use crate::services::{WeatherService, Weather};
use chrono::{Local, DateTime};
use std::collections::HashMap;
use reqwest;

pub struct WeatherAPICom {}

impl WeatherService for WeatherAPICom {
    fn get_weather_current(city: String, s_key: String) -> Result<Weather, ()> {
        #[derive(Deserialize, Serialize, Debug, Clone)]
        struct WeatherAPIComResponse {
            current: OpenWeatherMapCurrent,
        }

        #[derive(Deserialize, Serialize, Debug, Clone)]
        struct OpenWeatherMapCurrent {
            temp_c: f64
        }

        let mut query_params = HashMap::new();
        query_params.insert("q", city);
        query_params.insert("key", s_key);

        let res = reqwest::blocking::Client::new().get("http://api.weatherapi.com/v1/current.json")
            .query(&query_params)
            .send()
            .unwrap()
            .json::<WeatherAPIComResponse>()
            .unwrap();

        Ok(
            Weather {
                date: Local::now(),
                temperature: res.current.temp_c
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