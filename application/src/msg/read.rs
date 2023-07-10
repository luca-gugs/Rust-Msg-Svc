use domain::models::Msg;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn list_msg(msg_id: i32) -> Result<Msg, NotFound<String>> {
    use domain::schema::msgs;

    match msgs::table.find(msg_id).first::<Msg>(&mut establish_connection()) {
        Ok(msg) => Ok(msg),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting post with id {} - {}", msg_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}



pub fn list_msgs() -> Vec<Msg> {
    use domain::schema::msgs;
    match msgs::table.select(msgs::all_columns).load::<Msg>(&mut establish_connection()) {
        Ok(mut posts) => {
            posts.sort();
            posts
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
pub fn list_msgs_alt(roomid: String) -> Vec<Msg> {
    use domain::schema::msgs;
    match msgs::table.filter(msgs::roomid.eq(roomid)).load::<Msg>(&mut establish_connection()) {
        Ok(mut posts) => {
            posts.sort();
            posts
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

// pub fn list_msgs_alt(roomid: String) -> Result<Vec<Msg>, NotFound<String>> {
//     use domain::schema::msgs;

//     let connection = establish_connection();

//     let matching_msgs = msgs::table
//         .filter(msgs::roomid.eq(roomid))
//         .load::<Msg>(&connection);

//     match matching_msgs {
//         Ok(msgs) => {
//             if msgs.is_empty() {
//                 let response = Response {
//                     body: ResponseBody::Message(format!("No messages found with msg_id: {}", roomid)),
//                 };
//                 Err(NotFound(serde_json::to_string(&response).unwrap()))
//             } else {
//                 Ok(msgs)
//             }
//         }
//         Err(err) => {
//             panic!("Database error - {}", err);
//         }
//     }
// }
