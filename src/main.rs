// mod json;

extern crate eposlib;
#[macro_use]
extern crate rocket;

use std::env;

use eposlib::config::Amendment;
use eposlib::config::Config;
use eposlib::lm::LanguageModel;
// use std::fs::File;
use eposlib::lm;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::{Json, json, Value};
use rocket::State;

// use rocket::tokio::sync::Mutex;

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
async fn parse(p: Json<ParserInput>, lm: &State<LanguageModel>) -> Option<Json<Vec<String>>> {
    // let list = list.lock().await;

    // Some(Json(ParserInput {
    //     start_symbol: p.start_symbol,
    //     num_trees: p.num_trees,
    //     words: p.words,
    //     tags: p.tags,
    // }))
    // Some(p)
    None
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}


#[launch]
fn rocket() -> _ {
    let config = Config::new(env::args()).unwrap();
    // let lm = lm::load_model(&config).unwrap();

    rocket::build()
        .manage(lm::load_model(&config).unwrap())
        .mount("/", routes![parse])
        .register("/", catchers![not_found])
    // .launch()
    // .await;
    // rocket::build().attach(parse)
    // rocket::build()
    //     .mount("/", routes![parse])
    //     .register("/", catchers![not_found])
}

// #[rocket::main]
// async fn main() {
//     let config = Config::new(env::args()).unwrap();
//     // let lm = lm::load_model(&config).unwrap();
//
//     rocket::build()
//         .manage(lm::load_model(&config).unwrap())
//         .mount("/", routes![parse])
//         .register("/", catchers![not_found])
//         .launch()
//         .await;
// }

// use std::io;
//
// use rocket::tokio::task::spawn_blocking;
//
// #[get("/blocking_task")]
// async fn blocking_task() -> io::Result<Vec<u8>> {
//     // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
//     let vec = spawn_blocking(|| std::fs::read("data.txt")).await
//         .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;
//
//     Ok(vec)
// }