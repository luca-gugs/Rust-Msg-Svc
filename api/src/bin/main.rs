
#[macro_use] extern crate rocket;
use api::msg_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            msg_handler::list_msgs_handler, 
            msg_handler::list_msg_handler,
            msg_handler::create_msg_handler,
            msg_handler::list_msg_by_room,
        ])
}