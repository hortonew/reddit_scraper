#[macro_use]
extern crate rocket;

use analyzer::get_analysis;

#[get("/")]
fn index() -> String {
    get_analysis()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
