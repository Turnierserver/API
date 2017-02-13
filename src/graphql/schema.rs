use diesel::prelude::*;
use juniper::{ID, FieldResult};

use super::{Context, id};

use models::*;
use schema::users::dsl::users;
use schema::ais::dsl::ais;

graphql_object!(Context: Context |&self| {
    field user_store() -> UserStore { UserStore {} }
    field ai_store() -> AIStore { AIStore {} }

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


    field ais(&executor) -> Vec<AI> {
        AI::belonging_to(self)
            .load(&executor.context().conn)
            .unwrap() // FIXME
    }
});


struct AIStore {}
graphql_object!(AIStore: Context as "AIStore" |&self| {
    field ais(&executor) -> Vec<AI> {
        ais.load::<AI>(&executor.context().conn).unwrap()
    }
});

graphql_object!(AI: Context as "AI" |&self| {
    field id() -> ID { id("ai", self.id) }
    field name() -> &String { &self.name }
    field description() -> Option<&String> { self.description.as_ref() }
    field elo() -> f64 { self.elo }

    field user(&executor) -> User {
        users.find(self.user_id) // FIXME
            .first(&executor.context().conn)
            .unwrap() // FIXME
    }
});
