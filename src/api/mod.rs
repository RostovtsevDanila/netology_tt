use actix_web::{Responder, HttpRequest, HttpResponse};
use crate::services::openweathermap::OpenWeatherMap;
use crate::services::WeatherService;

pub struct NetologyTTApi {

}

impl NetologyTTApi {
    pub async fn get_weather_today() -> impl Responder {
        let weather = OpenWeatherMap::get_weather_current("Samara".to_string()).unwrap();
        println!("{:#?}", weather);
        HttpResponse::Ok()
    }

    pub async fn get_weather_week_ahead() -> impl Responder {
        HttpResponse::Ok()
    }
}