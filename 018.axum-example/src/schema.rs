table! {
    users (id) {
        id -> Integer,
        uuid -> Varchar,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        nick_name -> Varchar,
        head_img -> Varchar,
        authority_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
