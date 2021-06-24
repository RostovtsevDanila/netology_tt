mod services;
mod api;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;


use actix_web::{App, HttpServer, web, middleware};
use std::sync::Arc;
use std::env;


#[derive(Clone)]
pub struct EnvData {
    pub openweathermap_key: String,
    pub weatherapicom_key: String,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Service started!");

    let env_data = Arc::new(EnvData {
        openweathermap_key: env::var("OPENWEATHERMAP_KEY").expect("Couldn't read OPENWEATHERMAP_KEY"),
        weatherapicom_key: env::var("WEATHERAPICOM_KEY").expect("Couldn't read WEATHERAPICOM_KEY")
    });

    HttpServer::new(move || {
        App::new()
            .data(env_data.clone())
            .wrap(middleware::Logger::default())
            .route("/api/weather", web::get().to(api::NetologyTTApi::get_weather))
            .route("/api/weather/week_ahead", web::get().to(api::NetologyTTApi::get_weather_week_ahead))
    })
        .workers(4)
        .bind("0.0.0.0:9998")?
        .run()
        .await
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::{test, web, App};
//
//     #[actix_rt::test]
//     async fn get_weather_test() {
//         // std::thread::spawn(||{main().await});
//         main();
//         let client = reqwest::Client::new();
//
//         // Act
//         let response = client
//             .get("http://localhost:9998/api/weather&city=Samara")
//             .send()
//             .await
//             .expect("Failed to execute request.");
//     }
//
//
// }
