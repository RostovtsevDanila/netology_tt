use crate::services::{WeatherService, Weather};
use chrono::{NaiveDate, NaiveDateTime};
use std::collections::{HashMap, BTreeMap};
use async_trait::async_trait;


#[derive(Deserialize, Clone)]
struct NominatimResponse {
    lon: String,
    lat: String,
}

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


pub struct OpenWeatherMap {}

impl OpenWeatherMap {
    pub async fn get_weather_in_day(city: String, date: Option<NaiveDate>, s_key: String) -> Result<Weather, ()> {
        let weathers = Self::get_weather(city, s_key).await.unwrap();
        Ok(Self::get_weather_in_date(weathers, date).await)
    }

    pub async fn get_weather_week_ahead(city: String, s_key: String) -> Result<BTreeMap<NaiveDate, f64>, ()> {
        Ok(Self::get_weather(city, s_key).await.unwrap())
    }

    async fn get_city_coords(city: String) -> NominatimResponse {
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
}

#[async_trait]
impl WeatherService for OpenWeatherMap {
    async fn get_weather(city: String, s_key: String) -> Result<BTreeMap<NaiveDate, f64>, ()> {
        let coords = Self::get_city_coords(city).await;
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
