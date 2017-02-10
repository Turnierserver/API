#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate juniper;
extern crate turnierserver;

use rocket_contrib::{JSON, Template};
use turnierserver::graphql::{GraphqlQuery, GraphqlResult};

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn graphiql() -> Template {
    let data: Option<String> = None;
    Template::render("graphiql", &data)
}

#[get("/graphql?<query>")]
fn get_graphql(query: &str) -> GraphqlResult {
    let q = GraphqlQuery {
        query: query.into(),
        variables: None
    };
    q.execute()
}

#[post("/graphql", data = "<query>")]
fn post_graphql(query: JSON<GraphqlQuery>) -> GraphqlResult {
    query.execute()
}

#[get("/status")]
fn status() -> String {
    format!("TODO")
}

fn main() {
    rocket::ignite().mount("/", routes![
        hello,
        get_graphql,
        post_graphql,
        graphiql,
        status
    ]).launch();
}