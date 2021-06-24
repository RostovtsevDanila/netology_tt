use actix_web::{Responder, HttpRequest, HttpResponse, web};
use crate::services::openweathermap::OpenWeatherMap;
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
    city: String,
    date: Option<NaiveDate>,
}

pub struct NetologyTTApi {}

impl NetologyTTApi {
    pub async fn get_weather(req: HttpRequest, env_data: web::Data<Arc<EnvData>>) -> impl Responder {
        let query_params = Query::<QueryParams>::from_query(&req.query_string()).unwrap();
        let weathers = vec![
            OpenWeatherMap::get_weather_in_day(query_params.city.clone(), query_params.date.clone(),env_data.openweathermap_key.clone()).unwrap(),
            WeatherAPICom::get_weather_in_day(query_params.city.clone(), query_params.date.clone(),env_data.weatherapicom_key.clone()).unwrap(),
        ];
        let res_weather = weathers.iter().map(|w| w.temperature()).sum::<f64>() / weathers.len() as f64;
        HttpResponse::Ok().json(json!({"date": weathers[0].date(), "today_weather": res_weather}))
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