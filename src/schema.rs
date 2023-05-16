// @generated automatically by Diesel CLI.

diesel::table! {
    persona (id) {
        id -> Int4,
        username -> Varchar,
        name -> Varchar,
        surname -> Varchar,
        birthday -> Varchar,
    }
}

diesel::table! {
    personas (id) {
        id -> Int4,
        name -> Varchar,
        surname -> Text,
        birthday -> Date,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    persona,
    personas,
    posts,
    users,
);
