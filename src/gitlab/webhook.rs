use actix_web::{web, Error, HttpResponse, Responder};
use gitlab::webhooks;
use wecom::webhook as wc;

// #[post("/webhook")]
pub async fn webhook(
    body: web::Json<webhooks::WebHook>,
    key: web::Data<&str>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", body);
    let result = wc::new(key.get_ref()).notify_text("hello world!").await;
    println!("result: {:?}", result);
    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("ok")),
        Err(e) => {
            println!("{:?}", e);
            Ok(HttpResponse::InternalServerError().body(""))
        }
    }
}
