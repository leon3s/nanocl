//! Functions to manipulate clusters in database
use ntex::web;
use uuid::Uuid;
use diesel::prelude::*;

use crate::utils::get_pool_conn;
use crate::controllers::errors::HttpError;
use crate::repositories::errors::db_blocking_error;
use crate::models::{Pool, ClusterItem, ClusterPartial, PgDeleteGeneric};

/// # Create cluster for namespace
/// Return a fresh cluster with id and gen_id for given namespace
///
/// # Arguments
///
/// - [nsp](String) namespace of the cluster
/// - [item](ClusterPartial) - Cluster to create without id and other generated data
/// - [pool](web::types::State<Pool>) - Posgresql database pool
///
/// # Examples
///
/// ```
/// // Create a simple cluster
///
/// use crate::repositories::cluster;
/// let nsp = String::from("default");
/// let new_cluster = ClusterCreate {
///  name: String::from("test-cluster")
/// }
/// cluster::create_for_namespace(nsp, new_cluster, &pool).await;
/// ```
pub async fn create_for_namespace(
  nsp: String,
  item: ClusterPartial,
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
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

/// Return found cluster or an error otherwise
///
/// # Arguments
///
/// * `id` - Id of the cluster
/// * `pool` - Posgresql database pool
///
/// # Examples
///
/// ```
/// // Find cluster by id
///
/// use crate::repositories::cluster;
/// cluster::find_by_id(id, &pool).await;
/// ```
pub async fn find_by_id(
  id: Uuid,
  pool: &web::types::State<Pool>,
) -> Result<ClusterItem, HttpError> {
  use crate::schema::clusters::dsl;

  let conn = get_pool_conn(pool)?;
  let res =
    web::block(move || dsl::clusters.filter(dsl::id.eq(id)).get_result(&conn))
      .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

/// Return found cluster or an error otherwise
///
/// # Arguments
///
/// * `gen_id` - Generated id of the cluster
/// * `pool` - Posgresql database pool
///
/// # Examples
///
/// ```
/// // Find cluster by id
///
/// use crate::repositories::cluster;
/// cluster::find_by_gen_id(gen_id, &pool).await;
/// ```
pub async fn find_by_gen_id(
  gen_id: String,
  pool: &web::types::State<Pool>,
) -> Result<ClusterItem, HttpError> {
  use crate::schema::clusters::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    dsl::clusters
      .filter(dsl::gen_id.eq(gen_id))
      .get_result(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

/// Return list of cluster of given namespace
///
/// # Arguments
///
/// * `nsp` - Namespace name
/// * `pool` - Posgresql database pool
///
/// # Examples
///
/// ```
/// // List cluster by namespace
///
/// use crate::repositories::cluster;
/// cluster::find_by_namespace(gen_id, &pool).await;
/// ```
pub async fn find_by_namespace(
  nsp: String,
  pool: &web::types::State<Pool>,
) -> Result<Vec<ClusterItem>, HttpError> {
  use crate::schema::clusters::dsl::*;

  let conn = get_pool_conn(pool)?;
  let res =
    web::block(move || clusters.filter(namespace.eq(nsp)).load(&conn)).await;
  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(items) => Ok(items),
  }
}

/// Return number of deleted entries
///
/// # Arguments
///
/// * `gen_id` - Generated id of the cluster to delete
/// * `pool` - Posgresql database pool
///
/// # Examples
///
/// ```
/// // Delete cluster by generated id
///
/// use crate::repositories::cluster;
/// cluster::delete_by_gen_id(gen_id, &pool).await;
/// ```
pub async fn delete_by_gen_id(
  gen_id: String,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
  use crate::schema::clusters::dsl;

  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    diesel::delete(dsl::clusters)
      .filter(dsl::gen_id.eq(gen_id))
      .execute(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
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

    let _res = find_by_namespace(String::from("default"), &pool_state)
      .await
      .unwrap();
  }
}
