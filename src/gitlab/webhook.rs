use actix_web::{web, Error, HttpResponse};
use gitlab::{types, webhooks};
use wecom::webhook as wc;

// #[post("/webhook")]
pub async fn webhook(
    body: web::Json<webhooks::WebHook>,
    key: web::Data<String>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", body);
    let msg = match body.0 {
        webhooks::WebHook::MergeRequest(mr) => {
            let mra = mr.object_attributes;
            let action = match mra.action.unwrap_or(webhooks::MergeRequestAction::Open) {
                webhooks::MergeRequestAction::Open => "🥚 MR New",
                webhooks::MergeRequestAction::Update => "🐣 MR Update",
                webhooks::MergeRequestAction::Close  => "🦆 MR Close",
                webhooks::MergeRequestAction::Reopen => "🐥 MR Reopen",
                webhooks::MergeRequestAction::Approved => "🐤 MR Approved",
                webhooks::MergeRequestAction::Unapproved => "🐦 MR Unapproved",
                webhooks::MergeRequestAction::Merge => "🐔 MR Merge",
            };
            format!(
                "# {}: [{}]({})\n- assigned: <@{}>",
                action,
                mra.title,
                mra.url.unwrap(),
                mra.assignee_id.unwrap_or(types::UserId::new(0)).value(),
            )
        }
        _ => format!(":checkered_flag:"),
    };
    let result = wc::new(key.get_ref()).notify_markdown(&msg).await;
    println!("result: {:?}", result);
    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("ok")),
        Err(e) => {
            println!("{:?}", e);
            Ok(HttpResponse::InternalServerError().body(""))
        }
    }
}
