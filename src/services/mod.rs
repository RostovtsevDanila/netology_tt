pub mod openweathermap;
pub mod weatherapicom;

use chrono::{NaiveDate, Local};
use std::collections::BTreeMap;
use async_trait::async_trait;
use std::fmt::Formatter;
use reqwest::Error;


#[derive(Debug)]
pub enum  ServiceError {
    WeatherServiceError,
    NominatimServiceError,
    ExternalServiceError(reqwest::Error),
    DateError,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ServiceError::WeatherServiceError => write!(f, "Can't get weather."),
            ServiceError::NominatimServiceError => write!(f, "Can't get city coordinates."),
            ServiceError::ExternalServiceError(ref e) => e.fmt(f),
            ServiceError::DateError => write!(f, "Can't get weather this date."),
        }
    }
}

impl From<reqwest::Error> for ServiceError {
    fn from(err: Error) -> Self {
        ServiceError::ExternalServiceError(err)
    }
}

#[derive(Debug)]
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

#[async_trait]
trait WeatherService {
    async fn get_weather(city: String, s_key: String) -> Result<BTreeMap<NaiveDate, f64>, ServiceError>;
    async fn get_weather_in_date(weathers: BTreeMap<NaiveDate, f64>, date: Option<NaiveDate>) -> Result<Weather, ServiceError> {
        let date = match date {
            Some(d) => d,
            None => Local::now().naive_utc().date()
        };
        let temperature = match weathers.get(&date) {
            Some(t) => t.clone(),
            None => return Err(ServiceError::DateError)
        };
        Ok(Weather{date, temperature})
    }
}


#[cfg(test)]
pub mod weather_service_tests {
    use super::*;
    use std::option::Option::Some;

    struct TempWeatherService;

    #[async_trait]
    impl WeatherService for TempWeatherService {
        #[allow(dead_code)]
        async fn get_weather(_city: String, _s_key: String) -> Result<BTreeMap<NaiveDate, f64>, ServiceError> {
            todo!()
        }
    }

    #[actix_rt::test]
    async fn get_weather_in_date_test() {
        let mut weathers: BTreeMap<NaiveDate, f64> = BTreeMap::new();
        weathers.insert(NaiveDate::from_ymd(2021, 06, 01), 35.0);
        weathers.insert(NaiveDate::from_ymd(2021, 06, 02), 34.0);
        weathers.insert(NaiveDate::from_ymd(2021, 06, 03), 33.0);
        weathers.insert(NaiveDate::from_ymd(2021, 06, 04), 32.0);
        weathers.insert(NaiveDate::from_ymd(2021, 06, 05), 31.0);

        let w: Weather = TempWeatherService::get_weather_in_date(weathers, Some(NaiveDate::from_ymd(2021, 06, 03))).await.unwrap();
        assert_eq!(w.temperature, 33.0)
    }
}