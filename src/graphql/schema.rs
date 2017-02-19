use diesel::prelude::*;
use juniper::{ID, FieldResult};

use super::{Context, id};

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

    field me(&executor) -> Option<&User> {
        executor.context().user.as_ref()
    }
});


pub struct Mutation;
graphql_object!(Mutation: Context as "Mutation" |&self| {
    field test_mutate(new_val: i64) -> FieldResult<bool> {
        Ok(new_val > 1)
    }

    field auth_pw(&executor, username: String, password: String) -> FieldResult<String> {
        let user = User::named(username, &executor.context().conn)
            .map_err(|_| "unauthorized".to_owned())?;

        if user.verify_pass(&*password) {
            let token = executor.context().try(|conn| user.token(conn).map(
                |token| token.hyphenated().to_string()
            ))?;
            executor.context().set_token_cookie(&*token);
            Ok(token)
        } else {
            println!("invalid password");
            Err("unauthorized".to_owned())
        }
    }
});

struct UserStore;
graphql_object!(UserStore: Context as "UserStore" |&self| {
    field users(&executor) -> FieldResult<Vec<User>> {
        executor.context().try(|conn| users.load(conn))
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

    field secret(&executor) -> FieldResult<String> {
        if executor.context().can_access_user(&self) {
            Ok("foobar".to_owned())
        } else {
            Err("Can't touch this".to_owned())
        }
    }

    field firstname(&executor) -> Option<&String> {
        if executor.context().can_access_user(&self) || self.name_public {
            self.firstname.as_ref()
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
    field id() -> ID { id("ai", self.id) }
    field name() -> &String { &self.name }
    field description() -> Option<&String> { self.description.as_ref() }
    field elo() -> f64 { self.elo }

    field user(&executor) -> FieldResult<User> {
        executor.context().try(|conn| users.find(self.user_id).first(conn))
    }

    field gametype(&executor) -> FieldResult<GameType> {
        executor.context().try(|conn|
            gametypes.find(self.gametype_id) // FIXME
                .first(conn)
        )
    }

    field games(&executor) -> FieldResult<Vec<AiGameAssocs>> {
        executor.context().try(|conn|
            ai_game_assocs.filter(schema::ai_game_assocs::columns::ai_id.eq(self.id))
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
    field id() -> ID { id("gametype", self.id) }
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
    field id() -> ID { id("game", self.id) }

    field gametype(&executor) -> FieldResult<GameType> {
        executor.context().try(|conn|
            gametypes.find(self.gametype_id).first(conn)
        )
    }

    field ais(&executor) -> FieldResult<Vec<AiGameAssocs>> {
        executor.context().try(|conn|
            ai_game_assocs.filter(schema::ai_game_assocs::columns::game_id.eq(self.id))
                .load(conn)
        )
    }
});

graphql_object!(AiGameAssocs: Context as "AiGameConnection" |&self| {
    field id() -> ID { id("aigameassoc", self.id) }
    field score() -> Option<i64> { self.score.map(|v| v as _) }
    field rank() -> Option<i64> { self.rank.map(|v| v as _) }

    field game(&executor) -> FieldResult<Game> {
        executor.context().try(|conn| games.find(self.game_id).first(conn))
    }

    field ai(&executor) -> FieldResult<Ai> {
        executor.context().try(|conn| ais.find(self.ai_id).first(conn))
    }
});