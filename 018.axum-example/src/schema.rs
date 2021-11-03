table! {
    users (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        uuid -> Nullable<Varchar>,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        nick_name -> Nullable<Varchar>,
        head_img -> Nullable<Varchar>,
        authority_id -> Nullable<Varchar>,
    }
}
