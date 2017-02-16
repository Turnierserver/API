use diesel;
use diesel::SaveChangesDsl;
use diesel::pg::PgConnection;
use super::*;

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[has_many(ais)]
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
    pub token: Option<Uuid>
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
}