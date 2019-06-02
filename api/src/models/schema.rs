table! {
    groups (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
        name -> Nullable<Varchar>,
    }
}

table! {
    users_groups (id) {
        id -> Uuid,
        added_at -> Timestamptz,
        user_id -> Uuid,
        group_id -> Uuid,
    }
}

joinable!(users_groups -> groups (group_id));
joinable!(users_groups -> users (user_id));

allow_tables_to_appear_in_same_query!(
    groups,
    users,
    users_groups,
);
