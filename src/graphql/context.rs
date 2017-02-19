use diesel::prelude::*;
use diesel::pg::PgConnection;
use juniper;
use juniper::FieldResult;
use rocket::http::Cookies;
use uuid::Uuid;
use std::fmt::Debug;
use std::cell::RefCell;

use models::*;
use schema::users;
use schema::users::dsl::*;
use schema::tokens::dsl::*;
use establish_connection;

pub struct Context {
    pub user: Option<User>,
    pub conn: PgConnection,
    pub set_cookies: RefCell<Vec<(String, String)>>
}

impl Context {
    pub fn new(cookies: &Cookies) -> Self {
        let conn = establish_connection(); // TODO: r2d2-diesel

        let user = cookies.find("token")
            .and_then(|token| Uuid::parse_str(token.value()).ok())
            .and_then(|token| tokens.find(token).first::<Token>(&conn).ok())
            .and_then(|token| {
                users
                    .filter(users::columns::id.eq(token.user_id))
                    .first::<User>(&conn)
                    .ok()
            });

        Context {
            user: user,
            conn: conn,
            set_cookies: RefCell::new(Vec::new())
        }
    }

    pub fn can_access_user(&self, user: &User) -> bool {
        self.user.as_ref()
            .map(|u| u.admin || u.id == user.id)
            .unwrap_or(false)
    }

    pub fn can_access_ai(&self, ai: &Ai) -> bool {
        self.user.as_ref()
            .map(|u| u.admin || u.id == ai.user_id)
            .unwrap_or(false)
    }

    pub fn try<T, F, E>(&self, func: F) -> FieldResult<T>
        where F: Fn(&PgConnection) -> Result<T, E>,
            E: Debug {
        func(&self.conn).map_err(|e| {
            println!("{:?}", e);
            "database failure".to_owned()
        })
    }

    pub fn set_token_cookie(&self, token: &str) {
        self.set_cookies.borrow_mut()
            .push(("token".into(), token.into()))
    }
}

impl juniper::Context for Context {}