#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate juniper;
extern crate turnierserver;

use rocket_contrib::JSON;
use rocket::response::NamedFile;
use turnierserver::{GraphqlQuery, GraphqlResult};
use turnierserver::cors::Cors;

#[get("/")]
fn graphiql() -> NamedFile {
    NamedFile::open("static/graphiql.html").unwrap()
}

#[get("/?<query>")]
fn graphiql_w_query(query: &str) -> NamedFile {
    let _ = query; // TODO: DRY
    NamedFile::open("static/graphiql.html").unwrap()
}

#[get("/graphql?<query>")]
fn get_graphql(query: &str) -> Cors<GraphqlResult> {
    let q = GraphqlQuery {
        query: query.into(),
        variables: None
    };
    Cors(q.execute())
}

#[post("/graphql", data = "<query>")]
fn post_graphql(query: JSON<GraphqlQuery>) -> Cors<GraphqlResult> {
    Cors(query.execute())
}

#[get("/status")]
fn status() -> String {
    format!("TODO")
}

fn main() {
    rocket::ignite().mount("/", routes![
        get_graphql,
        post_graphql,
        graphiql,
        graphiql_w_query,
        status
    ]).launch();
}