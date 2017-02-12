use super::schema::*;

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
    pub name_public: bool,
    pub admin: bool,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub pwhash: &'a str,
    pub admin: bool,
}

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[table_name="ais"]
#[belongs_to(User)]
pub struct AI {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub elo: f64,
}
