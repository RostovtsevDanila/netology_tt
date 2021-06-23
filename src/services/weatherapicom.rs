use crate::services::{WeatherService, WeatherCurrent};
use chrono::{Local, NaiveDate};
use std::collections::{HashMap, BTreeMap};
use reqwest;

pub struct WeatherAPICom {}

impl WeatherService for WeatherAPICom {
    fn get_weather_current(city: String, s_key: String) -> Result<WeatherCurrent, ()> {
        #[derive(Deserialize)]
        struct WeatherAPIComResponse {
            current: Current,
        }

        #[derive(Deserialize)]
        struct Current {
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
            WeatherCurrent {
                datetime: Local::now(),
                temperature: res.current.temp_c
            }
        )
    }


    fn get_weather_week_ahead(city: String, s_key: String) -> Result<BTreeMap<NaiveDate, f64>, ()> {
        #[derive(Deserialize)]
        struct WeatherAPIComResponse {
            forecast: Forecast,
        }

        #[derive(Deserialize)]
        struct Forecast {
            forecastday: Vec<Forecastday>
        }

        #[derive(Deserialize)]
        struct Forecastday {
            date: NaiveDate,
            day: Day
        }

        #[derive(Deserialize)]
        struct Day {
            avgtemp_c: f64
        }

        let mut query_params = HashMap::new();
        query_params.insert("q", city);
        query_params.insert("key", s_key);
        query_params.insert("days", "5".to_string());

        let res = reqwest::blocking::Client::new().get("http://api.weatherapi.com/v1/forecast.json")
            .query(&query_params)
            .send()
            .unwrap()
            .json::<WeatherAPIComResponse>()
            .unwrap();

        let weathers = res.forecast.forecastday.iter().map(|a|{
            (a.date, a.day.avgtemp_c)
        }).collect::<BTreeMap<NaiveDate, f64>>();

        Ok(weathers)
    }
}