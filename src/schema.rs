table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    posts_tags (id) {
        id -> Uuid,
        post_id -> Nullable<Uuid>,
        tag_id -> Nullable<Uuid>,
    }
}

table! {
    tags (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

joinable!(posts_tags -> posts (post_id));
joinable!(posts_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    posts,
    posts_tags,
    tags,
);
