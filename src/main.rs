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

use juniper::{Context, FieldResult, FieldError, Value};
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
    pub dbPool: db::Pool,
}

impl Context for GraphQLContext {}

type Schema = RootNode<'static, GraphQLContext, EmptyMutation<GraphQLContext>>;

graphql_object!(User: () | &self | {
    field id() -> &i32 {
        &self.id
    }

    field name() -> &String {
        &self.name
    }
});

graphql_object!(GraphQLContext: GraphQLContext as "Query" |&self| {
    description: "The root query object of the schema"

    field users(&executor) -> FieldResult<Vec<User>> {
        match executor.context().dbPool.get() {
            Ok(conn) =>
                Ok(User::all(&conn)),
            Err(_) =>
                Err(FieldError::new("DB not available", Value::null())),
        }
    }
});

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// #[get("/users")]
// fn users(conn: db::Conn) -> Json<Vec<User>> {
//     let users = User::all(&conn);

//     Json(users)
// }

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
                        request: juniper_rocket::GraphQLRequest,
                        schema: State<Schema>)
                        -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    let routes = routes![index, graphiql, get_graphql_handler, post_graphql_handler];

    let dbPool = db::init_pool();

    let context = GraphQLContext {
        dbPool: dbPool,
    };

    // let schema = Schema::new(
    //     context,
    //     EmptyMutation::<GraphQLContext>::new()
    // );

    rocket::ignite()
        // .manage(dbPool)
        .manage(context)
        .manage(schema)
        .mount("/", routes)
        .launch();
}
