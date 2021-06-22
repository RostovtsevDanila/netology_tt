mod services;
mod api;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;


use actix_web::{App, HttpServer, web, middleware};
use std::env;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Service started!");
    HttpServer::new(move || {
        App::new()
            // .app_data(web::PayloadConfig::new(2 * 1024 * 1024))
            .wrap(middleware::Logger::default())
            .route("/api/weather/today", web::get().to(api::NetologyTTApi::get_weather_today))
            .route("/api/weather/week_ahead", web::get().to(api::NetologyTTApi::get_weather_week_ahead))
    })
        .workers(4)
        .bind("0.0.0.0:9999")?
        .run()
        .await
}