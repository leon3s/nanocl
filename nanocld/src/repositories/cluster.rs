use ntex::web;
use uuid::Uuid;
use diesel::prelude::*;

use crate::utils::get_pool_conn;
use crate::controllers::errors::HttpError;
use crate::repositories::errors::db_blocking_error;
use crate::models::{
    Pool,
    ClusterItem,
    ClusterCreate,
    PgDeleteGeneric,
};

pub async fn create_for_namespace(
    nsp: String,
    item: ClusterCreate,
    pool: &web::types::State<Pool>,
) -> Result<ClusterItem, HttpError> {
    use crate::schema::clusters::dsl::*;
    let conn = get_pool_conn(pool)?;

    let res = web::block(move || {
        let genid = nsp.to_owned() + "-" + &item.name;
        let new_cluster = ClusterItem {
            id: Uuid::new_v4(),
            name: item.name,
            gen_id: genid,
            namespace: nsp,
        };

        diesel::insert_into(clusters)
            .values(&new_cluster)
            .execute(&conn)?;
        Ok(new_cluster)
    }).await;

    match res {
        Err(err) => Err(db_blocking_error(err)),
        Ok(item) => Ok(item),
    }
}

pub async fn find_by_id(
    id: Uuid,
    pool: &web::types::State<Pool>,
) -> Result<ClusterItem, HttpError> {
    use crate::schema::clusters::dsl;

    let conn = get_pool_conn(pool)?;
    let res = web::block(move || {
        dsl::clusters.filter(dsl::id.eq(id)).get_result(&conn)
    }).await;

    match res {
        Err(err) => Err(db_blocking_error(err)),
        Ok(item) => Ok(item),
    }
}

pub async fn find_by_gen_id(
    gen_id: String,
    pool: &web::types::State<Pool>,
) -> Result<ClusterItem, HttpError> {
    use crate::schema::clusters::dsl;

    let conn = get_pool_conn(pool)?;
    let res = web::block(move || {
        dsl::clusters.filter(dsl::gen_id.eq(gen_id)).get_result(&conn)
    }).await;

    match res {
        Err(err) => Err(db_blocking_error(err)),
        Ok(item) => Ok(item),
    }
}

pub async fn find_by_namespace(
    nsp: String,
    pool: &web::types::State<Pool>,
) -> Result<Vec<ClusterItem>, HttpError> {
    use crate::schema::clusters::dsl::*;

    let conn = get_pool_conn(pool)?;
    let res = web::block(move || {
        clusters.filter(namespace.eq(nsp))
        .load(&conn)
    }).await;
    match res {
        Err(err) => Err(db_blocking_error(err)),
        Ok(items) => Ok(items),
    }
}

pub async fn delete_for_gen_id(
    gen_id: String,
    pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
    use crate::schema::clusters::dsl;

    let conn = get_pool_conn(pool)?;
    let res = web::block(move || {
        diesel::delete(dsl::clusters)
        .filter(dsl::gen_id.eq(gen_id))
        .execute(&conn)
    }).await;

    match res {
        Err(err) => Err(db_blocking_error(err)),
        Ok(result) => Ok(PgDeleteGeneric {
            count: result,
        })
    }
}

#[cfg(test)]
mod test_cluster {
    use ntex::web;

    use crate::postgre;

    use super::*;

    #[ntex::test]
    async fn main() {
        let pool = postgre::create_pool();
        let pool_state = web::types::State::new(pool);

        let res = find_by_namespace(String::from("default"), &pool_state).await.unwrap();
        assert!(res.is_empty());
    }
}
