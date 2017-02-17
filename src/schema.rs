table! {
    ai_game_assocs (id) {
        id -> Int4,
        game_id -> Int4,
        ai_id -> Int4,
        rank -> Nullable<Int4>,
        score -> Nullable<Int4>,
    }
}

table! {
    ais (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        elo -> Float8,
        gametype_id -> Int4,
    }
}

table! {
    games (id) {
        id -> Int4,
        timestamp -> Timestamptz,
        gametype_id -> Int4,
    }
}

table! {
    gametypes (id) {
        id -> Int4,
        name -> Varchar,
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
