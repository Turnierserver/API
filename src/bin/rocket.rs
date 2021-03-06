#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate juniper;
extern crate turnierserver;

use rocket_contrib::JSON;
use rocket::response::NamedFile;
use rocket::http::Cookies;
use turnierserver::rocket_glue::*;
use turnierserver::cors::Cors;

#[get("/")]
fn graphiql() -> NamedFile {
    NamedFile::open("static/graphiql.html").unwrap()
}

#[get("/?<query>")]
fn graphiql_w_query(query: &str) -> NamedFile {
    let _ = query;
    NamedFile::open("static/graphiql.html").unwrap()
}

#[options("/graphql")]
fn options_graphql<'a>() -> Cors<&'a str> {
    Cors("")
}

#[get("/graphql?<query>")]
fn get_graphql(cookies: &Cookies, query: &str) -> Cors<GraphqlResult> {
    let q = GraphqlQuery {
        query: query.into(),
        variables: None
    };
    let context = Context::new(cookies);
    Cors(q.execute(context, cookies))
}

#[post("/graphql", data = "<query>")]
fn post_graphql(cookies: &Cookies, query: JSON<GraphqlQuery>) -> Cors<GraphqlResult> {
    let context = Context::new(cookies);
    Cors(query.execute(context, cookies))
}

fn main() {
    let _ = turnierserver::establish_connection();
    rocket::ignite().mount("/", routes![
        get_graphql,
        post_graphql,
        options_graphql,
        graphiql,
        graphiql_w_query
    ]).launch();
}