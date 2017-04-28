use super::super::schema::*;

#[derive(Insertable)]
#[table_name="users"]
pub struct User<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub admin: bool,
}