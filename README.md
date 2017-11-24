# Rocket with GraphQl

- https://rocket.rs
- https://github.com/graphql-rust/juniper_rocket

Run:

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
