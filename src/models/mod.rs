use super::schema::*;
use uuid::Uuid;
use chrono::offset::utc::UTC;
use chrono::datetime::DateTime;
use bcrypt;

mod user;
pub use self::user::User;
pub mod insert;

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[belongs_to(User, GameType)]
#[has_many(versions, foreign_key="ai_id")]
#[has_many(ai_game_assocs, foreign_key="ai_id")]
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
#[belongs_to(GameType)]
#[has_many(ai_game_assocs, foreign_key="game_id")]
#[table_name="games"]
pub struct Game {
    pub id: i32,
    pub timestamp: DateTime<UTC>,
    pub gametype_id: i32,
}


#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[belongs_to(Game, Ai)]
#[table_name="ai_game_assocs"]
pub struct AiGameAssocs {
    pub id: i32,
    pub game_id: i32,
    pub ai_id: i32,
    pub score: Option<i32>,
    pub rank: Option<i32>,
}


#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[table_name="versions"]
#[belongs_to(Ai, Lang)]
pub struct AiVersion {
    pub id: i32,
    pub ai_id: i32,
    pub lang_id: i32,
    pub compiled: bool,
    pub qualified: bool,
    pub published: bool,
}

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[has_many(versions, foreign_key="lang_id")]
#[table_name="langs"]
pub struct Lang {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Identifiable, Insertable, Associations, AsChangeset)]
#[belongs_to(User)]
#[table_name="tokens"]
pub struct Token {
    pub id: Uuid,
    pub user_id: i32,
    pub timestamp: DateTime<UTC>
}