#[macro_use]
extern crate rocket;
use analyzer::get_analysis;
use rocket::config::Config;
use rocket::response::content::RawJson;

#[get("/")]
fn index() -> RawJson<String> {
    match get_analysis() {
        Ok(posts) => {
            let json = serde_json::to_string(&posts).unwrap_or_else(|_| "[]".to_string());
            RawJson(json)
        }
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
