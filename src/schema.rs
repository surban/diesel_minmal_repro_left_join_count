table! {
    posts (id) {
        id -> Text,
        title -> Text,
        user_id -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        name -> Text,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
