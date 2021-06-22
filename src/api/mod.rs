use actix_web::{Responder, HttpRequest, HttpResponse};

pub struct NetologyTTApi {

}

impl NetologyTTApi {
    pub async fn get_weather_today() -> impl Responder {
        HttpResponse::Ok()
    }

    pub async fn get_weather_week_ahead() -> impl Responder {
        HttpResponse::Ok()
    }
}