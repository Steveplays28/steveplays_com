use backend::routes::index::{self};
use rocket::fs::relative;
use rocket::fs::FileServer;

#[macro_use]
extern crate rocket;

#[get("/status")]
fn status() -> &'static str {
    "OK"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![status])
        .mount("/index/", routes![index::projects])
        .mount(
            "/projects",
            FileServer::from(relative!("resources/projects/")),
        )
}
