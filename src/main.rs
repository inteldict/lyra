// mod json;

extern crate eposlib;
#[macro_use]
extern crate rocket;

use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::{Json, json, Value};
use rocket::State;
use rocket::tokio::sync::Mutex;

use eposlib::config::Amendment;
use eposlib::lm::ChartAmendment;
use eposlib::config::Config;
use std::env;
use std::io::BufReader;
use std::fs::File;
use eposlib::lm;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct ParserInput {
    start: Option<String>,
    num: usize,
    words: Vec<String>,
    tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct ElisionInput {
    p: ParserInput,
    amendments: Vec<Amendment>,
}

#[get("/parse", format = "json", data = "<p>")]
async fn parse(p: Json<ParserInput>) -> Option<Json<ParserInput>> {
    // let list = list.lock().await;

    // Some(Json(ParserInput {
    //     start_symbol: p.start_symbol,
    //     num_trees: p.num_trees,
    //     words: p.words,
    //     tags: p.tags,
    // }))
    Some(p)
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}


// #[launch]
// fn rocket() -> _ {
//     // rocket::build().attach(parse)
//     rocket::build()
//         .mount("/", routes![parse])
//         .register("/", catchers![not_found])
// }

#[rocket::main]
async fn main() {
    let config = Config::new(env::args()).unwrap();
    let lm = lm::load_model(&config).unwrap();

    rocket::build()
        .mount("/", routes![parse])
        .register("/", catchers![not_found])
        .launch()
        .await;
}