use uuid::Uuid;
use diesel::prelude::*;

use crate::models::{
  GitRepositoryItem,
  GitRepositoryCreate,
};

pub fn create(
  item: GitRepositoryCreate,
  conn: &PgConnection,
) -> Result<GitRepositoryItem, diesel::result::Error>{
  use crate::schema::git_repositories::dsl::*;

  let mut gen_uname = item.namespace.to_owned();
  gen_uname.push('_');
  gen_uname.push_str(&item.name);

  let new_namespace = GitRepositoryItem {
    id: Uuid::new_v4(),
    name: item.name,
    namespace: item.namespace,
    uname: gen_uname,
    url: item.url,
    token: item.token.ok_or("").unwrap_or_else(|_| String::from("")),
  };

  diesel::insert_into(git_repositories)
  .values(&new_namespace)
  .execute(conn)?;
  Ok(new_namespace)
}

pub fn find_all(
  conn: &PgConnection,
) -> Result<Vec<GitRepositoryItem>, diesel::result::Error>{
  use crate::schema::git_repositories::dsl::*;

  let items = git_repositories.load::<GitRepositoryItem>(conn)?;
  Ok(items)
}
