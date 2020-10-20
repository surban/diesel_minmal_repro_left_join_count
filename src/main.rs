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

  let select = (users::all_columns, count(posts::user_id));

  let query = users::table
    .select(select)
    .left_join(posts::table.on(posts::user_id.eq(posts::id)));

  let r = query.load::<(User, i64)>(&conn).unwrap();

  println!("{}", r)
}
