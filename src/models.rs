use super::schema::*;
use uuid::Uuid;
use chrono::offset::utc::UTC;
use chrono::datetime::DateTime;

#[derive(Debug, Queryable, Identifiable, Associations, Insertable, AsChangeset)]
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
    pub token: Option<Uuid>
}

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[table_name="ais"]
#[belongs_to(User)]
#[belongs_to(GameType)]
pub struct Ai {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub elo: f64,
    pub gametype_id: i32,
}

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[table_name="gametypes"]
pub struct GameType {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[table_name="games"]
pub struct Game {
    pub id: i32,
    pub timestamp: DateTime<UTC>,
    pub gametype_id: i32,
}

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[table_name="ai_game_assocs"]
pub struct AiGameAssocs {
    pub id: i32,
    pub game_id: i32,
    pub ai_id: i32,
    pub score: Option<i32>,
    pub rank: Option<i32>,
}