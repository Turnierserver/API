use std::collections::HashMap;
use rocket_contrib::{JSON, Value};
use rocket::response::status;
use rocket::http::Status;
use juniper::{RootNode, Variables, execute};
use juniper::tests::model::Database;
use juniper::EmptyMutation;


#[derive(Debug, Serialize, Deserialize)]
pub struct GraphqlQuery {
    pub query: String,
    pub variables: Option<Variables>
}

pub type GraphqlResult = status::Custom<JSON<Value>>;

impl GraphqlQuery {
    pub fn execute(&self) -> GraphqlResult {
        let query = Database::new();
        let mutation = EmptyMutation::<Database>::new();
        let root = RootNode::new(query, mutation);
        let vars = self.variables.clone().unwrap_or(HashMap::new());
        let result = execute(self.query.as_str(), None, &root, &vars, &Database::new());
        match result {
            Ok((result, errors)) => {
                if errors.is_empty() {
                    status::Custom(Status::Ok, JSON(json!({
                        "data": result
                    })))
                } else {
                    status::Custom(Status::BadRequest, JSON(json!({
                        "data": result,
                        "errors": errors
                    })))
                }
            },
            Err(err) => {
                status::Custom(Status::BadRequest, JSON(json!({
                    "errors": err
                })))
            }
        }
    }
}
