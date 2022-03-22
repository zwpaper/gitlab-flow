use std::env;

use actix_web::{web, App, HttpServer};

mod gitlab;

use crate::gitlab::webhook::webhook;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let key = match env::var("GF_WECOM_WEBHOOK_KEY") {
        Ok(val) => val,
        Err(e) => panic!("{}", e),
    };
    HttpServer::new(move || {
        App::new().service(
            web::resource("/webhook")
                .app_data(web::Data::new(key.clone()))
                .route(web::post().to(webhook)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
