#[macro_use]
extern crate rocket;
use analyzer::{get_analysis, sentiment_label};
use models::ApiResponse;
use rocket::config::Config;
use rocket::response::content::RawJson;
use rocket_cors::{AllowedOrigins, CorsOptions};

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
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .expect("error creating CORS fairing");

    let config = Config {
        address: "0.0.0.0".parse().unwrap(),
        port: 8000,
        ..Config::default()
    };

    rocket::custom(config)
        .mount("/", routes![index])
        .attach(cors)
}
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::{http::Status, local::blocking::Client};

    #[test]
    fn test_index_returns_data() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let response = client.get("/").dispatch();
        let status = response.status();
        let body = response.into_string().unwrap();

        println!("Response status: {:?}", status);
        println!("Response body: {}", body);

        assert_eq!(status, Status::Ok);
        assert!(!body.is_empty(), "Expected response body to contain data");
    }
}
