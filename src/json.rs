// use std::borrow::Cow;
//
// use eposlib::lm::ChartAmendment;
// use rocket::serde::{Deserialize, Serialize};
// use rocket::serde::json::{Json, json, Value};
// use rocket::State;
// use rocket::tokio::sync::Mutex;
// use eposlib::config::Amendment;
//
// // We're going to store all of the messages here. No need for a DB.
// // type MessageList = Mutex<Vec<String>>;
// // type Messages<'r> = &'r State<MessageList>;
//
// // #[derive(Serialize, Deserialize)]
// // #[serde(crate = "rocket::serde")]
// // struct Message<'r> {
// //     id: Option<Id>,
// //     message: Cow<'r, str>,
// // }
//
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(crate = "rocket::serde")]
// struct ParserInput {
//     start: Option<String>,
//     num: usize,
//     words: Vec<String>,
//     tags: Option<Vec<String>>,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(crate = "rocket::serde")]
// struct ElisionInput {
//     p: ParserInput,
//     amendments: Vec<Amendment>,
// }
//
// // #[post("/", format = "json", data = "<message>")]
// // async fn new(message: Json<Message<'_>>, list: Messages<'_>) -> Value {
// //     let mut list = list.lock().await;
// //     let id = list.len();
// //     list.push(message.message.to_string());
// //     json!({ "status": "ok", "id": id })
// // }
//
// // #[put("/<id>", format = "json", data = "<message>")]
// // async fn update(id: Id, message: Json<Message<'_>>, list: Messages<'_>) -> Option<Value> {
// //     match list.lock().await.get_mut(id) {
// //         Some(existing) => {
// //             *existing = message.message.to_string();
// //             Some(json!({ "status": "ok" }))
// //         }
// //         None => None
// //     }
// // }
//
//
// # [get("/parse", format = "json", data="<p>")]
// async fn parse(p: Json<ParserInput>) -> Option<Json<ParserInput>> {
//     // let list = list.lock().await;
//
//     // Some(Json(ParserInput {
//     //     start_symbol: p.start_symbol,
//     //     num_trees: p.num_trees,
//     //     words: p.words,
//     //     tags: p.tags,
//     // }))
//     Some(p)
// }
//
// // #[get(" / elision", format = "json")]
// // async fn get(id: Id, list: Messages<'_>) -> Option<Json<Message<'_>>> {
// //     let list = list.lock().await;
// //
// //     Some(Json(Message {
// //         id: Some(id),
// //         message: list.get(id)?.to_string().into(),
// //     }))
// // }
//
// #[catch(404)]
// fn not_found() -> Value {
//     json!({
//         "status": "error",
//         "reason": "Resource was not found."
//     })
// }
//
// pub fn stage() -> rocket::fairing::AdHoc {
//     rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
//         rocket.mount("/", routes![parse])
//             .register("/", catchers![not_found])
//             // .manage(MessageList::new(vec![]))
//     })
// }
