use crate::services::{WeatherService, Weather, ServiceError};
use chrono::{NaiveDate};
use std::collections::{HashMap, BTreeMap};
use reqwest;
use async_trait::async_trait;


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


pub struct WeatherAPICom;

impl WeatherAPICom {
    pub async fn get_weather_in_day(city: String, date: Option<NaiveDate>, s_key: String) -> Result<Weather, ServiceError> {
        let weathers = Self::get_weather(city, s_key).await?;
        Ok(Self::get_weather_in_date(weathers, date).await?)
    }

    pub async fn get_weather_week_ahead(city: String, s_key: String) -> Result<BTreeMap<NaiveDate, f64>, ServiceError> {
        Ok(Self::get_weather(city, s_key).await?)
    }
}

#[async_trait]
impl WeatherService for WeatherAPICom {
    async fn get_weather(city: String, s_key: String) -> Result<BTreeMap<NaiveDate, f64>, ServiceError> {
        let mut query_params = HashMap::new();
        query_params.insert("q", city);
        query_params.insert("key", s_key);
        query_params.insert("days", "5".to_string());

        let response = match reqwest::blocking::Client::new().get("http://api.weatherapi.com/v1/forecast.json")
            .query(&query_params)
            .send() {
            Ok(v) => v,
            Err(e) => return Err(ServiceError::ExternalServiceError(e))
        };
        let res = match response.json::<WeatherAPIComResponse>() {
            Ok(v) => v,
            Err(_) => return Err(ServiceError::WeatherServiceError)
        };

        let weathers = res.forecast.forecastday.iter().map(|a|{
            (a.date, a.day.avgtemp_c)
        }).collect::<BTreeMap<NaiveDate, f64>>();
        Ok(weathers)
    }
}


#[cfg(test)]
pub mod weatherapicom_tests {
    use super::*;

    mod get_weather_in_day_test {
        use super::*;
        use chrono::Local;

        #[actix_rt::test]
        async fn correct_city_without_day() -> Result<(), ()> {
            match WeatherAPICom::get_weather_in_day("Samara".to_string(), None, std::env::var("WEATHERAPICOM_KEY").unwrap()).await {
                Ok(_) => Ok(()),
                Err(_) => Err(())
            }
        }

        #[actix_rt::test]
        async fn incorrect_city_without_day() -> Result<(), ()> {
            match WeatherAPICom::get_weather_in_day("adafsdfasdf".to_string(), None, std::env::var("WEATHERAPICOM_KEY").unwrap()).await {
                Ok(_) => Err(()),
                Err(_) => Ok(())
            }
        }

        #[actix_rt::test]
        async fn correct_city_with_correct_day() -> Result<(), ()> {
            match WeatherAPICom::get_weather_in_day("Kazan".to_string(), Some(Local::now().naive_utc().date()), std::env::var("WEATHERAPICOM_KEY").unwrap()).await {
                Ok(_) => Ok(()),
                Err(_) => Err(())
            }
        }

        #[actix_rt::test]
        async fn correct_city_with_incorrect_day() -> Result<(), ()> {
            match WeatherAPICom::get_weather_in_day("Kazan".to_string(), Some(Local::now().naive_utc().date() - chrono::Duration::days(1)), std::env::var("WEATHERAPICOM_KEY").unwrap()).await {
                Ok(_) => Err(()),
                Err(_) => Ok(())
            }
        }
    }

    mod get_weather_week_ahead_test {
        use super::*;

        #[actix_rt::test]
        async fn correct_city_without_day() -> Result<(), ()> {
            match WeatherAPICom::get_weather_week_ahead("Samara".to_string(), std::env::var("WEATHERAPICOM_KEY").unwrap()).await {
                Ok(_) => Ok(()),
                Err(_) => Err(())
            }
        }

        #[actix_rt::test]
        async fn incorrect_city_without_day() -> Result<(), ()> {
            match WeatherAPICom::get_weather_week_ahead("adafsdfasdf".to_string(), std::env::var("WEATHERAPICOM_KEY").unwrap()).await {
                Ok(_) => Err(()),
                Err(_) => Ok(())
            }
        }
    }
}