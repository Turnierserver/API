pub mod rocket_glue;
mod context;
pub use self::context::Context;
mod schema;
pub use self::schema::*;

use juniper::ID;
use base64::encode;

pub fn id(pre: &str, index: i32) -> ID {
    ID::from(encode(format!("{}-{}", pre, index).as_bytes()))
}