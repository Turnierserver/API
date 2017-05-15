use super::*;

/*
#[derive(Debug)]
pub enum AiStatus {
    Edited,
    Compiled,
    Qualified,
    Published
}
*/

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
    // pub status: AiStatus
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

/*
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
*/
