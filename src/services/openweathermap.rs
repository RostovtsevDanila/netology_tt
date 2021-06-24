use crate::services::{WeatherService, Weather, ServiceError};
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


pub struct OpenWeatherMap;

impl OpenWeatherMap {
    pub async fn get_weather_in_day(city: String, date: Option<NaiveDate>, s_key: String) -> Result<Weather, ServiceError> {
        let weathers = Self::get_weather(city, s_key).await?;
        Ok(Self::get_weather_in_date(weathers, date).await?)
    }

    pub async fn get_weather_week_ahead(city: String, s_key: String) -> Result<BTreeMap<NaiveDate, f64>, ServiceError> {
        Ok(Self::get_weather(city, s_key).await?)
    }

    async fn get_city_coords(city: String) -> Result<NominatimResponse, ServiceError> {
        let mut query_params = HashMap::new();
        query_params.insert("q", city);
        query_params.insert("format", "json".to_string());

        let res = reqwest::blocking::Client::new().get("http://nominatim.openstreetmap.org/search")
            .query(&query_params)
            .send()?
            .json::<Vec<NominatimResponse>>()?;
            let res = res.get(0);

        match res {
            Some(v) => {
                Ok(v.clone())
            },
            None => {
                Err(ServiceError::NominatimServiceError)
            }
        }
    }
}

#[async_trait]
impl WeatherService for OpenWeatherMap {
    async fn get_weather(city: String, s_key: String) -> Result<BTreeMap<NaiveDate, f64>, ServiceError> {
        let coords = Self::get_city_coords(city).await?;
        let mut query_params = HashMap::new();
        query_params.insert("lat", coords.lat);
        query_params.insert("lon", coords.lon);
        query_params.insert("exclude", "current,minutely,hourly,alerts".to_string());
        query_params.insert("units", "metric".to_string());
        query_params.insert("appid", s_key);

        let response = match reqwest::blocking::Client::new().get("https://api.openweathermap.org/data/2.5/onecall")
            .query(&query_params)
            .send() {
            Ok(v) => v,
            Err(e) => return Err(ServiceError::ExternalServiceError(e))
        };
        let res = match response.json::<OpenWeatherMapResponse>() {
            Ok(v) => v,
            Err(_) => return Err(ServiceError::WeatherServiceError)
        };

        let weathers = res.daily.iter().map(|a|{
            (NaiveDateTime::from_timestamp(a.dt, 0).date(), a.temp.day)
        }).collect::<BTreeMap<NaiveDate, f64>>();
        Ok(weathers)
    }
}


#[cfg(test)]
pub mod openweathermap_tests {
    use super::*;

    mod get_city_coords_test {
        use super::*;

        #[actix_rt::test]
        async fn correct_city() -> Result<(), ()> {
            match OpenWeatherMap::get_city_coords("Samara".to_string()).await {
                Ok(_) => Ok(()),
                Err(_) => Err(())
            }
        }

        #[actix_rt::test]
        async fn incorrect_city() -> Result<(), ()> {
            match OpenWeatherMap::get_city_coords("hasdkfhk".to_string()).await {
                Ok(_) => Err(()),
                Err(e) => Ok(assert_eq!(e.to_string(), ServiceError::NominatimServiceError.to_string()))
            }
        }
    }

    mod get_weather_in_day_test {
        use super::*;
        use chrono::Local;

        #[actix_rt::test]
        async fn correct_city_without_day() -> Result<(), ()> {
            match OpenWeatherMap::get_weather_in_day("Samara".to_string(), None, std::env::var("OPENWEATHERMAP_KEY").unwrap()).await {
                Ok(_) => Ok(()),
                Err(_) => Err(())
            }
        }

        #[actix_rt::test]
        async fn incorrect_city_without_day() -> Result<(), ()> {
            match OpenWeatherMap::get_weather_in_day("adafsdfasdf".to_string(), None, std::env::var("OPENWEATHERMAP_KEY").unwrap()).await {
                Ok(_) => Err(()),
                Err(_) => Ok(())
            }
        }

        #[actix_rt::test]
        async fn correct_city_with_correct_day() -> Result<(), ()> {
            match OpenWeatherMap::get_weather_in_day("Kazan".to_string(), Some(Local::now().naive_utc().date()), std::env::var("OPENWEATHERMAP_KEY").unwrap()).await {
                Ok(_) => Ok(()),
                Err(_) => Err(())
            }
        }

        #[actix_rt::test]
        async fn correct_city_with_incorrect_day() -> Result<(), ()> {
            match OpenWeatherMap::get_weather_in_day("Kazan".to_string(), Some(Local::now().naive_utc().date() - chrono::Duration::days(1)), std::env::var("OPENWEATHERMAP_KEY").unwrap()).await {
                Ok(_) => Err(()),
                Err(_) => Ok(())
            }
        }
    }

    mod get_weather_week_ahead_test {
        use super::*;

        #[actix_rt::test]
        async fn correct_city_without_day() -> Result<(), ()> {
            match OpenWeatherMap::get_weather_week_ahead("Samara".to_string(), std::env::var("OPENWEATHERMAP_KEY").unwrap()).await {
                Ok(_) => Ok(()),
                Err(_) => Err(())
            }
        }

        #[actix_rt::test]
        async fn incorrect_city_without_day() -> Result<(), ()> {
            match OpenWeatherMap::get_weather_week_ahead("adafsdfasdf".to_string(), std::env::var("OPENWEATHERMAP_KEY").unwrap()).await {
                Ok(_) => Err(()),
                Err(_) => Ok(())
            }
        }
    }
}