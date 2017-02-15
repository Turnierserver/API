use diesel::prelude::*;
use juniper::{ID, FieldResult};

use super::{Context, id};

use models::*;
use schema;
use schema::users::dsl::users;
use schema::ais::dsl::ais;
use schema::gametypes::dsl::gametypes;
use schema::ai_game_assocs::dsl::ai_game_assocs;


graphql_object!(Context: Context |&self| {
    field user_store() -> UserStore { UserStore {} }
    field ai_store() -> AiStore { AiStore {} }
    field gametype_store() -> GameTypeStore { GameTypeStore {} }

    field me() -> Option<&User> { self.user.as_ref() }
});

struct UserStore {}
graphql_object!(UserStore: Context as "UserStore" |&self| {
    field users(&executor) -> Vec<User> {
        users.load::<User>(&executor.context().conn).unwrap()
    }
});

graphql_object!(User: Context as "User" |&self| {
    description: "Ein Turnierserver-Nutzer"

    field id() -> ID as "Eine einzigartige Identifikationsnummer des Nutzers" {
        id("user", self.id)
    }
    field username() -> &String { &self.username }
    field email() -> &String { &self.email }
    field admin() -> bool { self.admin }

    field secret() -> FieldResult<&String> {
        Err("Can't touch this".to_owned())
    }

    field firstname(&executor) -> Option<&String> {
        if executor.context().can_access_user(&self) || self.name_public {
            self.firstname.as_ref()
        } else {
            None
        }
    }

    field ais(&executor) -> Vec<Ai> {
        Ai::belonging_to(self)
            .load(&executor.context().conn)
            .unwrap() // FIXME
    }
});


struct AiStore {}
graphql_object!(AiStore: Context as "AiStore" |&self| {
    field ais(&executor) -> Vec<Ai> {
        ais.load::<Ai>(&executor.context().conn).unwrap()
    }
});

graphql_object!(Ai: Context as "Ai" |&self| {
    field id() -> ID { id("ai", self.id) }
    field name() -> &String { &self.name }
    field description() -> Option<&String> { self.description.as_ref() }
    field elo() -> f64 { self.elo }

    field user(&executor) -> User {
        users.find(self.user_id) // FIXME
            .first(&executor.context().conn)
            .unwrap() // FIXME
    }

    field gametype(&executor) -> GameType {
        gametypes.find(self.gametype_id) // FIXME
            .first(&executor.context().conn)
            .unwrap() // FIXME
    }
});

struct GameTypeStore {}
graphql_object!(GameTypeStore: Context as "GameTypeStore" |&self| {
    field gametypes(&executor) -> Vec<GameType> {
        gametypes.load::<GameType>(&executor.context().conn).unwrap()
    }
});

graphql_object!(GameType: Context as "GameType" |&self| {
    field id() -> ID { id("gametype", self.id) }
    field name() -> &String { &self.name }

    field ais(&executor) -> Vec<Ai> {
        /*
        Ai::belonging_to(self)
            .load(&executor.context().conn)
            .unwrap() // FIXME
        */
        ais.filter(schema::ais::columns::gametype_id.eq(self.id))
            .load(&executor.context().conn)
            .unwrap() // FIXME, belonging_to
    }
});