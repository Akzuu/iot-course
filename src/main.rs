use std::env;

#[macro_use] extern crate rocket;

#[get("/post")]
fn get_post_log() -> &'static str {
    "Hello, world!"
}

#[post("/post")]
fn notify_post() -> () {
    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_post_log, notify_post])
}
