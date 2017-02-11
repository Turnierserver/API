// infer_schema!("dotenv:DATABASE_URL");
table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        firstname -> Nullable<Varchar>,
        lastname -> Nullable<Varchar>,
        pwhash -> Text,
        admin -> Bool,
        name_public -> Bool,
    }
}