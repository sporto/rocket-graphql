use diesel::prelude::*;
use super::schema::users;

use diesel::pg::PgConnection;

#[derive(Serialize, Deserialize, Queryable)]
// #[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn all(conn: &PgConnection) -> Vec<User> {
        users::table.load::<User>(conn).unwrap()
    }
}
