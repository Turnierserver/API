use diesel::pg::PgConnection;
use diesel::prelude::*;
use juniper::*;

use establish_connection;
use models::*;
use schema::users::dsl::*;

pub struct Database {
    conn: PgConnection,
}

impl Database {
    pub fn new() -> Self {
        Database {
            conn: establish_connection()
        }
    }

    pub fn me<'a>(&self) -> Option<&User> {
        unimplemented!()
    }
}

impl Context for Database {}

graphql_object!(Database: Database |&self| {
    field user_store() -> UserStore {
        UserStore {
            users: users.load::<User>(&self.conn).unwrap()
        }
    }

    field me() -> FieldResult<User> {
        unimplemented!() // Ok(users.first::<User>(&self.conn).unwrap())
    }
});

#[derive(Debug)]
struct UserStore {
    users: Vec<User>
}

graphql_object!(UserStore: Database as "UserStore" |&self| {
    field users() -> &Vec<User> {
        &self.users
    }
});

graphql_object!(User: Database as "User" |&self| {
    description: "Ein Turnierserver-Nutzer"

    field id() -> String as "Eine einzigartige Identifikationsnummer des Nutzers" {
        format!("{}", self.id)
    }
    field username() -> &String { &self.username }
    field email() -> &String { &self.email }
    field admin() -> bool { self.admin }

    field secret() -> FieldResult<&String> {
        Err("Can't touch this".to_owned())
    }

    field firstname() -> Option<&String> {
        self.firstname.as_ref() // TODO: authentication
    }


    field ais(&executor) -> Vec<AI> {
        unimplemented!()//executor.context().conn
    }
});



graphql_object!(AI: Database as "AI" |&self| {
    field id() -> String {
        format!("{}", self.id)
    }

    field user(&executor) -> User {
        unimplemented!()
    }
});
