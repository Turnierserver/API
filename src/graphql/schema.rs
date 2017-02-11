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
}

impl Context for Database {}

graphql_object!(Database: Database |&self| {
    field users() -> Vec<User> {
        users.load::<User>(&self.conn).unwrap()
    }
});


graphql_object!(User: Database as "User" |&self| {
    description: "Ein Turnierserver-Nutzer"

    field id() -> i64 as "Eine einzigartige Identifikationsnummer des Nutzers" {
        self.id as i64
    }

    field username() -> &String {
        &self.username
    }

    field email() -> &String {
        &self.email
    }

    field admin() -> bool {
        self.admin
    }

    field secret() -> FieldResult<&String> {
        Err("Can't touch this".to_owned())
    }

    field firstname() -> Option<&String> {
        self.firstname.as_ref() // TODO: authentication
    }
});
