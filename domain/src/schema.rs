// @generated automatically by Diesel CLI.

diesel::table! {
    msgs (id) {
        id -> Int4,
        roomid -> Varchar,
        senderid -> Text,
        body -> Text,
    }
}
