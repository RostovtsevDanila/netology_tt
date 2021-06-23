use actix_web::{Responder, HttpRequest, HttpResponse, web};
use crate::services::openweathermap::OpenWeatherMap;
use crate::services::WeatherService;
use crate::services::weatherapicom::WeatherAPICom;
use crate::EnvData;
use std::sync::Arc;
use actix_web::web::Query;
use std::collections::BTreeMap;
use chrono::NaiveDate;


fn intersect_maps_by_key(maps: &Vec<BTreeMap<NaiveDate, f64>>) -> BTreeMap<NaiveDate, f64> {
    let mut max_map: BTreeMap<NaiveDate, f64> = maps[0].clone();
    let mut res_map: BTreeMap<NaiveDate, f64> = BTreeMap::new();

    for v in maps {
        if v.len() > max_map.len() {
            max_map = v.clone();
        }
    }

    for date in max_map.keys() {
        let mut divider = 1;
        let avg = maps.iter().map(|v| {
            match v.get(date) {
                Some(t) => {
                    divider += 1;
                    t.clone()
                },
                None => max_map.get(date).unwrap().clone()
            }
        })
            .collect::<Vec<f64>>()
            .iter().sum::<f64>() / maps.len() as f64;
        res_map.insert(date.clone(), avg);
    }

    res_map
}


#[derive(Deserialize)]
struct QueryParams {
    city: String
}

pub struct NetologyTTApi {}

impl NetologyTTApi {
    pub async fn get_weather_current(req: HttpRequest, env_data: web::Data<Arc<EnvData>>) -> impl Responder {
        let query_params = Query::<QueryParams>::from_query(&req.query_string()).unwrap();
        let weather = vec![
            OpenWeatherMap::get_weather_current(query_params.city.clone(), env_data.openweathermap_key.clone()).unwrap().temperature(),
            WeatherAPICom::get_weather_current(query_params.city.clone(), env_data.weatherapicom_key.clone()).unwrap().temperature(),
        ];
        let res_weather = weather.iter().sum::<f64>() / weather.len() as f64;
        HttpResponse::Ok().json(json!({"current_weather": res_weather}))
    }

    pub async fn get_weather_week_ahead(req: HttpRequest, env_data: web::Data<Arc<EnvData>> ) -> impl Responder {
        let query_params = Query::<QueryParams>::from_query(&req.query_string()).unwrap();
        let weathers = vec![
            WeatherAPICom::get_weather_week_ahead(query_params.city.clone(), env_data.weatherapicom_key.clone()).unwrap(),
            OpenWeatherMap::get_weather_week_ahead(query_params.city.clone(), env_data.openweathermap_key.clone()).unwrap(),
        ];
        let res_weathers = intersect_maps_by_key(&weathers);
        HttpResponse::Ok().json(res_weathers)
    }
}