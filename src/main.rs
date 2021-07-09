mod json;

#[macro_use]
extern crate rocket;
extern crate eposlib;

use rocket::serde::{Serialize, Deserialize, json::Json};

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/foo/<_>/bar")]
fn foo_bar() -> &'static str {
    "Foo _____ bar!"
}

#[get("/<_..>")]
fn everything() -> &'static str {
    "Hey, you're here."
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(json::stage())
    // rocket::build().mount("/", routes![hello, foo_bar, everything, json::stage()])
}

// #[rocket::main]
// async fn main() {
//     rocket::build()
//         .mount("/", routes![hello, foo_bar, everything])
//         .launch()
//         .await;
// }
// fn main() {}