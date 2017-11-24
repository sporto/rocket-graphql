#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate juniper;

extern crate rocket;
extern crate juniper_rocket;

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket::response::content;
use rocket::State;
use std::collections::HashMap;
use juniper::Context;
use juniper::{EmptyMutation, RootNode};

struct User {
    id: String,
    name: String,
}

struct Database {
    users: HashMap<String, User>,
}

impl Context for Database {}

type Schema = RootNode<'static, Database, EmptyMutation<Database>>;

graphql_object!(User: Database | &self | {
    // Expose a simple field as a GraphQL string.
    field id() -> &String {
        &self.id
    }

    field name() -> &String {
        &self.name
    }
});

graphql_object!(Database: Database as "Query" |&self| {
    description: "The root query object of the schema"

    field users(&executor) -> Vec<&User> {
        executor.context()
            .users
            .iter()
            .map(|(_, user)| user.clone() )
            .collect()
    }

    field user(&executor, id: String) -> Option<&User> {
        executor.context().users.get(&id)
    }
});

#[get("/graphiql")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(context: State<Database>,
                       request: juniper_rocket::GraphQLRequest,
                       schema: State<Schema>)
                       -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(context: State<Database>,
                        request: juniper_rocket::GraphQLRequest,
                        schema: State<Schema>)
                        -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn new_database() -> Database {
    let mut users = HashMap::new();

    users.insert("1".to_string(),
                 User {
                     id: "1".to_string(),
                     name: "Sam".to_string(),
                 });

    users.insert("2".to_string(),
                 User {
                     id: "2".to_string(),
                     name: "Sally".to_string(),
                 });

    Database { users: users }
}

fn main() {
    let routes = routes![graphiql, get_graphql_handler, post_graphql_handler];

    rocket::ignite()
        .manage(new_database())
        .manage(Schema::new(Database { users: HashMap::new() },
                            EmptyMutation::<Database>::new()))
        .mount("/", routes)
        .launch();
}
