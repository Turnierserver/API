use super::schema::*;
use uuid::Uuid;
use chrono::offset::utc::UTC;
use chrono::datetime::DateTime;
use bcrypt;

mod user;
pub use self::user::User;

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[belongs_to(User, GameType)]
#[table_name="ais"]
pub struct Ai {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub elo: f64,
    pub gametype_id: i32,
}

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[has_many(ais, foreign_key="gametype_id")]
#[has_many(games, foreign_key="gametype_id")]
#[table_name="gametypes"]
pub struct GameType {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[table_name="games"]
#[belongs_to(GameType)]
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