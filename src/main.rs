#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket_contrib;
extern crate rocket;
#[macro_use] extern crate juniper;
extern crate juniper_rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_diesel;

use juniper::Context;
use juniper::{EmptyMutation, RootNode};
use rocket::response::content;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::{Json};
use std::path::{Path, PathBuf};
use models::user::User;
use rocket::fairing::{Fairing,Kind,Info};
use rocket::request::{Request, FromRequest};

mod db;
mod models;

pub struct GraphQLContext {
    pub connection: db::Conn,
}

impl Context for GraphQLContext {}

type Schema = RootNode<'static, GraphQLContext, EmptyMutation<GraphQLContext>>;

graphql_object!(User: () | &self | {
    // Expose a simple field as a GraphQL string.
    field id() -> &i32 {
        &self.id
    }

    field name() -> &String {
        &self.name
    }
});

graphql_object!(GraphQLContext: GraphQLContext as "Query" |&self| {
    description: "The root query object of the schema"

    field users(&executor) -> Vec<User> {
        User::all(&executor.context().connection)
    }
});

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
fn users(conn: db::Conn) -> Json<Vec<User>> {
    let users = User::all(&conn);

    Json(users)
}

#[get("/graphiql")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(context: State<GraphQLContext>,
                       request: juniper_rocket::GraphQLRequest,
                       schema: State<Schema>)
                       -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(context: State<GraphQLContext>,
                        conn: db::Conn,
                        request: juniper_rocket::GraphQLRequest,
                        schema: State<Schema>)
                        -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    let pool = db::init_pool();

    // let schema = Schema::new(
    //     GraphQLContext { },
    //     EmptyMutation::<GraphQLContext>::new()
    // );

    let routes = routes![index, users, graphiql, get_graphql_handler, post_graphql_handler];

    rocket::ignite()
        .manage(pool)
        // .manage(new_graphql_context())
        // .manage(schema)
        .mount("/", routes)
        .launch();
}
