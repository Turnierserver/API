use super::schema::users;

#[derive(Debug, Queryable, Identifiable, AsChangeset)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub pwhash: String,
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