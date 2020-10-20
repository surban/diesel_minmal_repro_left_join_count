use crate::schema::*;

#[derive(Debug, Clone, Queryable, Insertable, Associations, Identifiable)]
pub struct User {
  pub id: String,
  pub name: String,
}

#[derive(Debug, Clone, Queryable, Insertable, Associations, Identifiable)]
#[belongs_to(User)]
#[primary_key(id)]
pub struct Post {
  pub id: String,
  pub title: String,
  pub user_id: String,
}
