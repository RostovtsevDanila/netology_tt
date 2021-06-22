mod services;

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
            // .route("/version", web::get().to()
    })
        .workers(4)
        .bind(env::var("SERVER_ADDRESS").unwrap_or("0.0.0.0:80".to_string()))?
        .run()
        .await
}