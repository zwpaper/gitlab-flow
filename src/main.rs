use actix_web::{web, App, HttpServer};

mod gitlab;
mod wecom;

use crate::gitlab::webhook::webhook;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let key: &'static str = "hello";
    HttpServer::new(move || {
        App::new().service(
            web::resource("/webhook")
                .app_data(web::Data::new(key))
                .route(web::post().to(webhook)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
