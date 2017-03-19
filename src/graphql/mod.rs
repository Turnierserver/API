#[macro_use] mod relay_macros;
pub mod rocket_glue;
mod context;
pub use self::context::Context;
mod query;
pub use self::query::*;
mod mutation;
pub use self::mutation::*;

use std::str;
use juniper::ID;
use base64;

pub enum IDKind {
    User, Ai, Gametype,
    Game, AiGameAssoc
}

impl IDKind {
    fn prefix(&self) -> &'static str {
        use self::IDKind::*;
        match *self {
            User => "user",
            Ai => "ai",
            Gametype => "gametype",
            Game => "game",
            AiGameAssoc => "aigameassoc"
        }
    }

    pub fn enc(&self, index: i32) -> ID {
        let s = format!("{}-{}", self.prefix(), index);
        ID::from(base64::encode(s.as_bytes()))
    }

    pub fn dec(&self, id: &ID) -> Result<i32, String> {
        let s = base64::decode(&*id).map_err(|_| "invalid base64".to_owned())?;
        let s = str::from_utf8(&s).map_err(|_| "invalid utf8 str".to_owned())?;
        let (kind, data) = s.split_at(self.prefix().len() + 1);
        if &kind[0..kind.len() - 1] != self.prefix() {
            Err(format!("wrong kind, expected {}", self.prefix()))
        } else {
            data.parse().map_err(|_| "expected number".to_owned())
        }
    }
}