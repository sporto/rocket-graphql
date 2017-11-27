# Rocket with GraphQl

- https://rocket.rs
- https://github.com/graphql-rust/juniper_rocket

## Setup

- Install direnv https://direnv.net/
- Copy .envrc.example -> .envrc

## Run

`cargo run`

Go to

http://localhost:8000/graphiql

Run a query like:

```
query   {
  users {
    name
  }
  user(id: "1") {
    name    
  }
}
```
