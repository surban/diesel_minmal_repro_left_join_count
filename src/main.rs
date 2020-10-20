#[macro_use]
extern crate diesel;

mod models;
mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::*;

fn main() {
  dotenv().ok();

  let database_url =
    env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let conn = PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url));

  use crate::schema::*;
  use diesel::dsl::count;

  let query = users::table
    .select((users::id, users::name, count(posts::user_id)))
    .left_join(posts::table.on(users::id.eq(posts::user_id)))
    .group_by(users::id);

  let r = query.load::<(User, i64)>(&conn).unwrap();

  println!("{:#?}", r)
}
