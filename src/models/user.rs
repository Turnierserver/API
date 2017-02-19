use diesel;
use diesel::insert;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use super::*;
use schema::users::dsl as user_dsl;
use schema::tokens::dsl as token_dsl;

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[has_many(ais)]
#[has_many(tokens)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub pwhash: Option<String>,
    pub admin: bool,
    pub name_public: bool,
}

impl User {
    pub fn verify_pass(&self, pw: &str) -> bool {
        self.pwhash.as_ref()
            .and_then(|hash| bcrypt::verify(pw, &*hash).ok())
            .unwrap_or(false)
    }

    pub fn set_pass(&mut self, pw: &str, conn: &PgConnection) -> Result<User, diesel::result::Error> {
        let hash = bcrypt::hash(pw, bcrypt::DEFAULT_COST).unwrap();
        self.pwhash = Some(hash);
        self.save_changes::<User>(conn)
    }

    pub fn token(&self, conn: &PgConnection) -> Result<Uuid, diesel::result::Error> {
        let user_tokens = token_dsl::tokens.filter(token_dsl::user_id.eq(self.id))
            .order(token_dsl::timestamp)
            .limit(1)
            .load::<Token>(conn)?;
        
        Ok(if user_tokens.is_empty() {
            let token = Uuid::new_v4();
            let new_token = Token {
                id: token,
                user_id: self.id,
                timestamp: UTC::now()
            };
            insert(&new_token).into(token_dsl::tokens).execute(conn)?;
            token
        } else {
            user_tokens[0].id
        })
    }

    pub fn named(name: String, conn: &PgConnection) -> Result<User, diesel::result::Error> {
        user_dsl::users.filter(user_dsl::username.eq(name))
            .first(conn)
    }
}