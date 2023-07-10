
use domain::models::Msg;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Msg(Msg),
    Msgs(Vec<Msg>)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
