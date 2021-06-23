use crate::services::{WeatherService, Weather};
use chrono::{Local, NaiveDate};
use std::collections::{HashMap, BTreeMap};
use reqwest;


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


pub struct WeatherAPICom {

}

impl WeatherAPICom {
    pub fn get_weather_in_day(city: String, date: Option<NaiveDate>, s_key: String) -> Result<Weather, ()> {
        let weathers = Self::get_weather(city, s_key).unwrap();
        let weather: Weather = match date {
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
        };

        Ok(weather)
    }

    pub fn get_weather_week_ahead(city: String, s_key: String) -> Result<BTreeMap<NaiveDate, f64>, ()> {
        Ok(Self::get_weather(city, s_key).unwrap())
    }
}

impl WeatherService for WeatherAPICom {
    fn get_weather(city: String, s_key: String) -> Result<BTreeMap<NaiveDate, f64>, ()> {
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