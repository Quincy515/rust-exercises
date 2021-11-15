table! {
    users (id) {
        id -> Unsigned<Bigint>,
        uuid -> Nullable<Varchar>,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        nick_name -> Nullable<Varchar>,
        head_img -> Nullable<Varchar>,
        authority_id -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}
