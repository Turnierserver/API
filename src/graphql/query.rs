#![allow(unknown_lints, redundant_closure_call)] // TODO: fix juniper
use diesel::prelude::*;
use juniper::{ID, FieldResult};

use super::{Context, IDKind};

use models::*;
use schema;
use schema::users::dsl::users;
use schema::ais::dsl::ais;
use schema::gametypes::dsl::gametypes;
use schema::games::dsl::games;
use schema::ai_game_assocs::dsl::ai_game_assocs;

pub struct Query;
graphql_object!(Query: Context as "Query" |&self| {
    field user_store() -> UserStore { UserStore }
    field ai_store() -> AiStore { AiStore }
    field game_store() -> GameStore { GameStore }
    field gametype_store() -> GameTypeStore { GameTypeStore }
});

struct UserStore;
graphql_object!(UserStore: Context as "UserStore" |&self| {
    field users(&executor) -> FieldResult<Vec<User>> {
        executor.context().try(|conn| users.load(conn))
    }

    field user(&executor, id: ID) -> FieldResult<User> {
        let id = IDKind::User.dec(&id)?;
        executor.context().try(|conn| {
            users.find(id).first(conn)
        })
    }

    field me(&executor) -> Option<&User> {
        executor.context().user.as_ref()
    }
});

graphql_object!(User: Context as "User" |&self| {
    description: "Ein Turnierserver-Nutzer"

    field id() -> ID as "Eine einzigartige Identifikationsnummer des Nutzers" {
        IDKind::User.enc(self.id)
    }
    field username() -> &String { &self.username }
    field email() -> &String { &self.email }
    field admin() -> bool { self.admin }

    field canEdit(&executor) -> bool { executor.context().can_access_user(&self) }

    field firstname(&executor) -> Option<&String> {
        if executor.context().can_access_user(&self) || self.name_public {
            self.firstname.as_ref()
        } else {
            None
        }
    }

    field lastname(&executor) -> Option<&String> {
        if executor.context().can_access_user(&self) || self.name_public {
            self.lastname.as_ref()
        } else {
            None
        }
    }

    field ais(&executor) -> FieldResult<Vec<Ai>> {
        executor.context().try(|conn| Ai::belonging_to(self).load(conn))
    }
});


struct AiStore;
graphql_object!(AiStore: Context as "AiStore" |&self| {
    field ais(&executor) -> FieldResult<Vec<Ai>> {
        executor.context().try(|conn| ais.load(conn))
    }
});

graphql_object!(Ai: Context as "Ai" |&self| {
    field id() -> ID { IDKind::Ai.enc(self.id) }
    field name() -> &String { &self.name }
    field description() -> Option<&String> { self.description.as_ref() }
    field elo() -> f64 { self.elo }
    field rank() -> i64 { 0 }
    field icon() -> &str { "https://i.imgur.com/OTtzg4F.png" /* TODO: implement icons */ }

    field user(&executor) -> FieldResult<User> {
        executor.context().try(|conn| users.find(self.user_id).first(conn))
    }

    field gametype(&executor) -> FieldResult<GameType> {
        executor.context().try(|conn|
            gametypes
                .find(self.gametype_id)
                .first(conn)
        )
    }

    field games(&executor) -> FieldResult<Vec<AiGameAssocs>> {
        executor.context().try(|conn|
            ai_game_assocs
                .filter(schema::ai_game_assocs::columns::ai_id.eq(self.id))
                .load(conn)
        )
    }
});

struct GameTypeStore;
graphql_object!(GameTypeStore: Context as "GameTypeStore" |&self| {
    field gametypes(&executor) -> FieldResult<Vec<GameType>> {
        executor.context().try(|conn| gametypes.load(conn))
    }
});

graphql_object!(GameType: Context as "GameType" |&self| {
    field id() -> ID { IDKind::Gametype.enc(self.id) }
    field name() -> &String { &self.name }

    field ais(&executor) -> FieldResult<Vec<Ai>> {
        // FIXME: belonging_to
        executor.context().try(|conn| ais.filter(schema::ais::columns::gametype_id.eq(self.id)).load(conn))
    }
});

struct GameStore;
graphql_object!(GameStore: Context as "GameStore" |&self| {
    field games(&executor) -> FieldResult<Vec<Game>> {
        executor.context().try(|conn| games.load(conn))
    }
});

graphql_object!(Game: Context as "Game" |&self| {
    field id() -> ID { IDKind::Game.enc(self.id) }

    field gametype(&executor) -> FieldResult<GameType> {
        executor.context().try(|conn|
            gametypes.find(self.gametype_id).first(conn)
        )
    }

    field timestamp(&executor) -> String {
        self.timestamp.to_string()
    }

    field ais(&executor) -> FieldResult<Vec<AiGameAssocs>> {
        executor.context().try(|conn|
            ai_game_assocs.filter(schema::ai_game_assocs::columns::game_id.eq(self.id))
                .load(conn)
        )
    }
});

graphql_object!(AiGameAssocs: Context as "AiGameConnection" |&self| {
    field id() -> ID { IDKind::AiGameAssoc.enc(self.id) }
    field score() -> Option<i64> { self.score.map(|v| v as _) }
    field rank() -> Option<i64> { self.rank.map(|v| v as _) }

    field game(&executor) -> FieldResult<Game> {
        executor.context().try(|conn| games.find(self.game_id).first(conn))
    }

    field ai(&executor) -> FieldResult<Ai> {
        executor.context().try(|conn| ais.find(self.ai_id).first(conn))
    }
});