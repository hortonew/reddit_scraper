#[macro_use]
extern crate rocket;
use analyzer::{get_analysis, sentiment_label};
use models::ApiResponse;
use rocket::config::Config;
use rocket::response::content::RawJson;

#[get("/")]
fn index() -> RawJson<String> {
    match get_analysis() {
        Ok(Some((post, sentiment_score))) => {
            let response = ApiResponse {
                title: post.title,
                selftext: post.selftext,
                created_utc: post.created_utc,
                url: post.url,
                sentiment_score,
                sentiment_label: sentiment_label(sentiment_score).to_string(),
            };
            let json = serde_json::to_string(&response).unwrap_or_else(|_| "[]".to_string());
            RawJson(json)
        }
        Ok(None) => RawJson("[]".to_string()),
        Err(e) => RawJson(format!(r#"{{"error": "{}"}}"#, e)),
    }
}

#[launch]
fn rocket() -> _ {
    let config = Config {
        address: "0.0.0.0".parse().unwrap(),
        port: 8000,
        ..Config::default()
    };
    rocket::custom(config).mount("/", routes![index])
}
