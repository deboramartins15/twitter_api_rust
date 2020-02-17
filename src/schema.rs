table! {
    tweets (id) {
        id -> Nullable<Int4>,
        user_id -> Int4,
        content -> Varchar,
    }
}

table! {
    users (id) {
        id -> Nullable<Int4>,
        name -> Varchar,
        username -> Nullable<Varchar>,
    }
}

joinable!(tweets -> users (user_id));

allow_tables_to_appear_in_same_query!(
    tweets,
    users,
);
