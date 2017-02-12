use std::collections::HashMap;
use rocket_contrib::{JSON, Value};
use rocket::response::status;
use rocket::http::Status;
use juniper::{RootNode, Variables, EmptyMutation, GraphQLType, execute};

pub use super::context::Context;

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphqlQuery {
    pub query: String,
    pub variables: Option<Variables>,
}

pub type GraphqlResult = status::Custom<JSON<Value>>;

impl GraphqlQuery {
    pub fn execute<CtxT, Q>(&self, query: Q, context: CtxT) -> GraphqlResult
        where Q: GraphQLType<Context=CtxT>,
            CtxT: GraphQLType {
        println!("{}", self.query);
        let mutation = EmptyMutation::<CtxT>::new();
        let root = RootNode::new(query, mutation);
        let vars = self.variables.clone().unwrap_or(HashMap::new());

        let result = execute(self.query.as_str(), None, &root, &vars, &context);

        match result {
            Ok((result, errors)) => {
                if errors.is_empty() {
                    status::Custom(Status::Ok, JSON(json!({
                        "data": result
                    })))
                } else {
                    println!("{}", self.query);
                    println!("{:?}", errors);
                    status::Custom(Status::BadRequest, JSON(json!({
                        "data": result,
                        "errors": errors
                    })))
                }
            },
            Err(err) => {
                println!("{}", self.query);
                println!("{:?}", err);
                status::Custom(Status::BadRequest, JSON(json!({
                    "errors": err
                })))
            }
        }
    }
}
