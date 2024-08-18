#[macro_use]
extern crate rocket;
use analyzer::{get_analysis, sentiment_label};
use models::ApiResponse;
use prometheus::{
    register_int_counter, register_int_counter_vec, Encoder, IntCounter, IntCounterVec, TextEncoder,
};
use rocket::config::Config;
use rocket::response::content::RawJson;
use rocket::State;
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::sync::Arc;

struct Metrics {
    health_check_counter: IntCounter,
    sentiment_counter: IntCounterVec,
}

#[get("/")]
fn index(metrics: &State<Arc<Metrics>>) -> RawJson<String> {
    metrics.health_check_counter.inc(); // Increment the health check counter

    match get_analysis() {
        Ok(Some((post, sentiment_score))) => {
            let sentiment_label = sentiment_label(sentiment_score).to_string();
            metrics
                .sentiment_counter
                .with_label_values(&[&sentiment_label])
                .inc(); // Increment the sentiment counter

            let response = ApiResponse {
                title: post.title,
                selftext: post.selftext,
                created_utc: post.created_utc,
                url: post.url,
                sentiment_score,
                sentiment_label,
            };
            let json = serde_json::to_string(&response).unwrap_or_else(|_| "[]".to_string());
            RawJson(json)
        }
        Ok(None) => RawJson("[]".to_string()),
        Err(e) => RawJson(format!(r#"{{"error": "{}"}}"#, e)),
    }
}

#[get("/metrics")]
fn metrics_endpoint(metrics: &State<Arc<Metrics>>) -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
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

    // Initialize Prometheus metrics
    let metrics = Arc::new(Metrics {
        health_check_counter: register_int_counter!(
            "api_health_check_total",
            "Total health checks"
        )
        .unwrap(),
        sentiment_counter: register_int_counter_vec!(
            "api_sentiment_total",
            "Total sentiment analysis",
            &["sentiment"]
        )
        .unwrap(),
    });

    rocket::custom(config)
        .manage(metrics)
        .mount("/", routes![index, metrics_endpoint])
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

    #[test]
    fn test_metrics_endpoint() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let response = client.get("/metrics").dispatch();
        let status = response.status();
        let body = response.into_string().unwrap();

        println!("Response status: {:?}", status);
        println!("Metrics body: {}", body);

        assert_eq!(status, Status::Ok);
        assert!(body.contains("api_health_check_total"));
        assert!(body.contains("api_sentiment_total"));
    }
}
