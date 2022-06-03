use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{GitRepositoryCreate, GitRepositoryItem};

pub fn create_for_namespace(
    nsp: String,
    item: GitRepositoryCreate,
    conn: &PgConnection,
) -> Result<GitRepositoryItem, diesel::result::Error> {
    use crate::repositories::namespace::find_by_id_or_name;
    use crate::schema::git_repositories::dsl::*;

    let resp = find_by_id_or_name(nsp, conn)?;

    let new_namespace = GitRepositoryItem {
        id: Uuid::new_v4(),
        name: item.name,
        namespace: resp.name,
        owner: item.owner,
        source: item.source,
        token: item.token.ok_or("").unwrap_or_else(|_| String::from("")),
    };

    diesel::insert_into(git_repositories)
        .values(&new_namespace)
        .execute(conn)?;
    Ok(new_namespace)
}

pub fn find_by_namespace(
    nsp: String,
    conn: &PgConnection,
) -> Result<Vec<GitRepositoryItem>, diesel::result::Error> {
    use crate::schema::git_repositories::dsl::*;

    let items = git_repositories.filter(namespace.eq(nsp)).load(conn)?;
    Ok(items)
}

// Not used for now
pub fn _find_all(conn: &PgConnection) -> Result<Vec<GitRepositoryItem>, diesel::result::Error> {
    use crate::schema::git_repositories::dsl::*;

    let items = git_repositories.load::<GitRepositoryItem>(conn)?;
    Ok(items)
}
