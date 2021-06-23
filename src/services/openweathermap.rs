use crate::services::{WeatherService, WeatherCurrent};
use chrono::{Local, NaiveDate, NaiveDateTime};
use std::collections::{HashMap, BTreeMap};

pub struct OpenWeatherMap {}

#[derive(Deserialize, Clone)]
struct NominatimResponse {
    lon: String,
    lat: String,
}

fn get_city_coords(city: String) -> NominatimResponse {
    let mut query_params = HashMap::new();
    query_params.insert("q", city);
    query_params.insert("format", "json".to_string());

    reqwest::blocking::Client::new().get("http://nominatim.openstreetmap.org/search")
        .query(&query_params)
        .send()
        .unwrap()
        .json::<Vec<NominatimResponse>>()
        .unwrap().get(0).unwrap().clone()
}


impl WeatherService for OpenWeatherMap {
    fn get_weather_current(city: String, s_key: String) -> Result<WeatherCurrent, ()> {
        #[derive(Deserialize)]
        struct OpenWeatherMapResponse {
            main: Main,
        }

        #[derive(Deserialize)]
        struct Main {
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
            WeatherCurrent {
                datetime: Local::now(),
                temperature: res.main.temp - 273.15
            }
        )
    }


    fn get_weather_week_ahead(city: String, s_key: String) -> Result<BTreeMap<NaiveDate, f64>, ()> {
        let coords = get_city_coords(city);

        #[derive(Deserialize)]
        struct OpenWeatherMapResponse {
            daily: Vec<Daily>,
        }

        #[derive(Deserialize)]
        struct Daily {
            dt: i64,
            temp: Temp,
        }

        #[derive(Deserialize)]
        struct Temp {
            day: f64,
        }

        let mut query_params = HashMap::new();
        query_params.insert("lat", coords.lat);
        query_params.insert("lon", coords.lon);
        query_params.insert("exclude", "current,minutely,hourly,alerts".to_string());
        query_params.insert("units", "metric".to_string());
        query_params.insert("appid", s_key);

        let res = reqwest::blocking::Client::new().get("https://api.openweathermap.org/data/2.5/onecall")
            .query(&query_params)
            .send()
            .unwrap()
            .json::<OpenWeatherMapResponse>()
            .unwrap();

        let weathers = res.daily.iter().map(|a|{
            (NaiveDateTime::from_timestamp(a.dt, 0).date(), a.temp.day)
        }).collect::<BTreeMap<NaiveDate, f64>>();

        Ok(weathers)
    }
}

