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

pub fn id(pre: &str, index: i32) -> ID {
    let s = format!("{}-{}", pre, index);
    ID::from(base64::encode(s.as_bytes()))
}

pub fn rid(pre: &str, id: &ID) -> Result<i32, String> {
    let s = base64::decode(&*id).map_err(|_| "invalid base64".to_owned())?;
    let s = str::from_utf8(&s).map_err(|_| "invalid kind str".to_owned())?;
    let (kind, data) = s.split_at(pre.len() + 1);
    if &kind[0..kind.len() - 1] != pre {
        Err(format!("wrong kind, expected {}", pre))
    } else {
        data.parse().map_err(|_| "expected number".to_owned())
    }
}