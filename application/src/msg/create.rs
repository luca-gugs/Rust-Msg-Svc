use domain::models::{Msg, NewMsg};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_msg(msg: Json<NewMsg>) -> Created<String> {
    use domain::schema::msgs;

    let msg: NewMsg = msg.into_inner();

    match diesel::insert_into(msgs::table).values(&msg).get_result::<Msg>(&mut establish_connection()) {
        Ok(post) => {
            let response = Response { body: ResponseBody::Msg(post) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}