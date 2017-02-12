use diesel::prelude::*;
use diesel::pg::PgConnection;
use juniper;
use rocket::http::Cookies;
use uuid::Uuid;

use models::*;
use schema::users;
use schema::users::dsl::*;
use establish_connection;

pub struct Context {
    pub user: Option<User>,
    pub conn: PgConnection,
}

impl Context {
    pub fn new(cookies: &Cookies) -> Self {
        let conn = establish_connection(); // TODO: r2d2-diesel

        let user = cookies.find("user")
            .and_then(|name| cookies.find("token").map(|_token| {
                let name = name.value().to_owned();
                let _token = _token.value().to_owned();
                (name, _token)
            }))
            .and_then(|(name, _token)| {
                let user = users
                    .filter(users::columns::username.eq(name))
                    .first::<User>(&conn)
                    .unwrap();

                if user.token == Uuid::parse_str(&*_token).ok() {
                    Some(user)
                } else {
                    None
                }
            });

        Context {
            user: user,
            conn: conn,
        }
    }

    pub fn can_access_user(&self, user: &User) -> bool {
        self.user.as_ref()
            .map(|u| u.admin || u.id == user.id)
            .unwrap_or(false)
    }

    pub fn can_access_ai(&self, ai: &AI) -> bool {
        self.user.as_ref()
            .map(|u| u.admin || u.id == ai.user_id)
            .unwrap_or(false)
    }
}

impl juniper::Context for Context {}