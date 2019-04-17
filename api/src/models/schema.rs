table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
        name -> Nullable<Varchar>,
    }
}
