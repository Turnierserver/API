use rocket_contrib::{JSON, Value};
use rocket::response::status;
use rocket::http::Status;
use rocket::http::{Cookie, Cookies};
use juniper::{RootNode, Variables, execute};

pub use super::context::Context;
use super::{Query, Mutation};

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphqlQuery {
    pub query: String,
    pub variables: Option<Variables>,
}

pub type GraphqlResult = status::Custom<JSON<Value>>;

impl GraphqlQuery {
    pub fn execute(&self, context: Context, cookie_jar: &Cookies) -> GraphqlResult {
        let root = RootNode::new(Query, Mutation);
        let vars = self.variables.clone().unwrap_or_default();

        let result = execute(self.query.as_str(), None, &root, &vars, &context);

        let Context { set_cookies, .. } = context;
        for (key, val) in set_cookies.into_inner() {
            cookie_jar.add(Cookie::new(key, val))
        }

        match result {
            Ok((result, errors)) => {
                if errors.is_empty() {
                    status::Custom(Status::Ok, JSON(json!({
                        "data": result
                    })))
                } else {
                    println!("{}", self.query);
                    println!("{:?}", self.variables);
                    println!("{:?}", errors);
                    status::Custom(Status::BadRequest, JSON(json!({
                        "data": result,
                        "errors": errors
                    })))
                }
            },
            Err(err) => {
                println!("{}", self.query);
                println!("{:?}", self.variables);
                println!("{:?}", err);
                status::Custom(Status::BadRequest, JSON(json!({
                    "errors": err
                })))
            }
        }
    }
}
