use super::super::schema::*;
use chrono::offset::utc::UTC;
use chrono::datetime::DateTime;

#[derive(Insertable)]
#[table_name="users"]
pub struct User<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub admin: bool,
}

#[derive(Insertable)]
#[table_name="ais"]
pub struct Ai<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub gametype_id: i32,
    pub user_id: i32,
}


#[derive(Insertable)]
#[table_name="games"]
pub struct Game {
    pub timestamp: DateTime<UTC>,
    pub gametype_id: i32,
}