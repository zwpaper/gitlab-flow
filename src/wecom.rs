use awc::Client;
use serde_json;

pub async fn notify(key: &str) {
    let request = serde_json::json!({
        "lang": "rust",
        "body": "json"
    });
    let client = Client::default();

    println!("{}", key);
    let res = client
        .post(format!(
            "https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key={}",
            key,
        ))
        .insert_header(("User-Agent", "Actix-web"))
        .insert_header(("Content-Type", "application/json"))
        .send_json(&request)
        .await;

    println!("Response: {:?}", res); // <- server http response
}
