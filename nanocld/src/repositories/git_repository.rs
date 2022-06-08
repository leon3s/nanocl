use ntex::web;
use uuid::Uuid;
use diesel::prelude::*;

use crate::utils::get_pool_conn;
use crate::controllers::errors::HttpError;
use crate::repositories::errors::db_blocking_error;
use crate::models::{GitRepositorySourceType, GitRepositoryCreate, GitRepositoryItem, Pool, PgDeleteGeneric};

fn gen_git_repository_url(item: &GitRepositoryCreate) -> String {
    "https://".to_owned() + match item.source {
        GitRepositorySourceType::Github => "github.com",
        GitRepositorySourceType::Gitlab => "gitlab.com",
        GitRepositorySourceType::Local => "localhost",
    } + "/" + &item.name + ".git"
}

/// Create git repository
pub async fn create(
    item: GitRepositoryCreate,
    pool: &web::types::State<Pool>,
) -> Result<GitRepositoryItem, HttpError> {
    use crate::schema::git_repositories::dsl;

    let conn = get_pool_conn(pool)?;
    let res = web::block(move || {
        let url = gen_git_repository_url(&item);
        let new_namespace = GitRepositoryItem {
            id: Uuid::new_v4(),
            name: item.name,
            gen_url: url,
            token: None,
            source: item.source,
        };
        diesel::insert_into(
            dsl::git_repositories
        )
        .values(&new_namespace)
        .execute(&conn)?;
        Ok(new_namespace)
    }).await;

    match res {
        Err(err) => Err(db_blocking_error(err)),
        Ok(item) => Ok(item),
    }
}

/// Delete git repository by id or name
pub async fn delete_by_id_or_name(
    id: String,
    pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
    use crate::schema::git_repositories::dsl;

    let conn = get_pool_conn(pool)?;
    let res = match Uuid::parse_str(&id) {
        Err(_) => web::block(move || {
            diesel::delete(dsl::git_repositories.filter(dsl::name.eq(id))).execute(&conn)
        }).await,
        Ok(uuid) => web::block(move || {
            diesel::delete(
                dsl::git_repositories
                .filter(dsl::id.eq(uuid))
            ).execute(&conn)
        }).await,
    };

    match res {
        Err(err) => Err(db_blocking_error(err)),
        Ok(result) => Ok(PgDeleteGeneric { count: result })
    }
}

/// List all git repository
pub async fn list(
    pool: &web::types::State<Pool>,
) -> Result<Vec<GitRepositoryItem>, HttpError> {
    use crate::schema::git_repositories::dsl;

    let conn = get_pool_conn(pool)?;
    let res = web::block(move || {
        dsl::git_repositories.load(&conn)
    }).await;
    match res {
        Err(err) => Err(db_blocking_error(err)),
        Ok(items) => Ok(items),
    }
}

#[cfg(test)]
mod test_git_repository {
    use crate::postgre;
    use crate::models::GitRepositorySourceType;

    use super::*;

    #[ntex::test]
    async fn main() {
        let pool = postgre::create_pool();
        let pool_state = web::types::State::new(pool);
        // Find
        let _res = list(
            &pool_state,
        ).await.unwrap();
        let item = GitRepositoryCreate {
            name: String::from("test"),
            token: Some(String::from("test")),
            source: GitRepositorySourceType::Github,
        };
        // Create
        let res = create(
            item,
            &pool_state,
        ).await.unwrap();
        assert_eq!(res.name, "test");

        // Delete with id
        let res = delete_by_id_or_name(
            res.id.to_string(),
            &pool_state,
        ).await.unwrap();
        assert_eq!(res.count, 1);
        let item = GitRepositoryCreate {
            name: String::from("test"),
            token: Some(String::from("test")),
            source: GitRepositorySourceType::Github,
        };
        let res = create(
            item,
            &pool_state,
        ).await.unwrap();
        let res = delete_by_id_or_name(
            res.name,
            &pool_state,
        ).await.unwrap();
        assert_eq!(res.count, 1);
    }
}
