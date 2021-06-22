use actix_web::{Responder, HttpRequest, HttpResponse, web};
use crate::services::openweathermap::OpenWeatherMap;
use crate::services::WeatherService;
use crate::services::weatherapicom::WeatherAPICom;
use crate::EnvData;
use std::sync::Arc;
use actix_web::web::Query;


#[derive(Deserialize)]
struct QueryParams {
    city: String
}


pub struct NetologyTTApi {}

impl NetologyTTApi {
    pub async fn get_weather_today(req: HttpRequest, env_data: web::Data<Arc<EnvData>>) -> impl Responder {
        let query_params = Query::<QueryParams>::from_query(&req.query_string()).unwrap();
        let weather_1 = OpenWeatherMap::get_weather_current(query_params.city.clone(), env_data.openweathermap_key.clone()).unwrap();
        let weather_2 = WeatherAPICom::get_weather_current(query_params.city.clone(), env_data.weatherapicom_key.clone()).unwrap();
        HttpResponse::Ok().json(json!({"weather_today_c": (weather_1.temperature() + weather_2.temperature()) / 2.0}))
    }

    pub async fn get_weather_week_ahead() -> impl Responder {
        HttpResponse::Ok()
    }
}