#[macro_use]
extern crate rocket;
use analyzer::get_analysis;
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
    rocket::build().mount("/", routes![index])
}
