table! {
    databases (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    databases,
    posts,
);
