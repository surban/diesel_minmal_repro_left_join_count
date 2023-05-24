use diesel::prelude::*;
use super::schema::users;
use super::schema::posts;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable)]
pub struct User {
  pub id: String,
  pub name: String,
}

#[derive(Debug, Clone, Queryable, Insertable, Associations, Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(primary_key(id))]
pub struct Post {
  pub id: String,
  pub title: String,
  pub user_id: String,
}
