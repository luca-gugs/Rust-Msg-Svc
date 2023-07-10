use crate::schema::msgs;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Msg {
    pub id: i32,
    pub roomid: String,
    pub senderid: String,
    pub body: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = msgs)]
pub struct NewMsg {
    pub roomid: String,
    pub senderid: String,
    pub body: String,
}