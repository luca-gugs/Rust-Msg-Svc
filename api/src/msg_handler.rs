// api/src/post_handler.rs

use shared::response_models::{Response, ResponseBody};

use application::msg::{create, read}; // 👈 New!

use domain::models::{Msg, NewMsg};
use rocket::{get, post};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;

#[get("/")]
pub fn list_msgs_handler() -> String {
    let msgs: Vec<Msg> = read::list_msgs();
    let response = Response { body: ResponseBody::Msgs(msgs) };

    serde_json::to_string(&response).unwrap()
}

#[get("/msg/<msg_id>")]
pub fn list_msg_handler(msg_id: i32) -> Result<String, NotFound<String>> {
    let msg = read::list_msg(msg_id)?;
    let response = Response { body: ResponseBody::Msg(msg) };
    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/msg/room/<room_id>")]
pub fn list_msg_by_room(room_id: String) -> Result<String, NotFound<String>> {
    let msg = read::list_msgs_alt(room_id);
    let response = Response { body: ResponseBody::Msgs(msg) };
    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/new_msg", format = "application/json", data = "<msg>")]
pub fn create_msg_handler(msg: Json<NewMsg>) -> Created<String> {
    create::create_msg(msg)
}