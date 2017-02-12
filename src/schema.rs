table! {
    ais (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        elo -> Float8,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        firstname -> Nullable<Varchar>,
        lastname -> Nullable<Varchar>,
        pwhash -> Nullable<Text>,
        admin -> Bool,
        name_public -> Bool,
        token -> Nullable<Uuid>,
    }
}