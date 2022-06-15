use ntex::web;
use uuid::Uuid;
use diesel::prelude::*;

use crate::utils::get_pool_conn;
use crate::controllers::errors::HttpError;
use crate::models::{
  Pool, ClusterNetworkPartial, ClusterNetworkItem, PgDeleteGeneric,
};

use super::errors::db_blocking_error;

pub async fn create_for_cluster(
  cluster_id: Uuid,
  item: ClusterNetworkPartial,
  pool: &web::types::State<Pool>,
) -> Result<ClusterNetworkItem, HttpError> {
  use crate::schema::cluster_networks::dsl;
  let conn = get_pool_conn(pool)?;

  let res = web::block(move || {
    let item = ClusterNetworkItem {
      id: Uuid::new_v4(),
      name: item.name,
      cluster_id,
    };
    diesel::insert_into(dsl::cluster_networks)
      .values(&item)
      .execute(&conn)?;
    Ok(item)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

pub async fn delete_by_id_or_name(
  id: String,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
  use crate::schema::cluster_networks::dsl;
  let conn = get_pool_conn(pool)?;
  let res = match Uuid::parse_str(&id) {
    Err(_) => {
      web::block(move || {
        diesel::delete(dsl::cluster_networks)
          .filter(dsl::name.eq(id))
          .execute(&conn)
      })
      .await
    }
    Ok(uuid) => {
      web::block(move || {
        diesel::delete(dsl::cluster_networks)
          .filter(dsl::id.eq(uuid))
          .execute(&conn)
      })
      .await
    }
  };

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}

pub async fn find_by_id_or_name(
  id: String,
  pool: &web::types::State<Pool>,
) -> Result<ClusterNetworkItem, HttpError> {
  use crate::schema::cluster_networks::dsl;
  let conn = get_pool_conn(pool)?;

  let res = match Uuid::parse_str(&id) {
    Err(_) => {
      web::block(move || {
        dsl::cluster_networks
          .filter(dsl::name.eq(id))
          .get_result(&conn)
      })
      .await
    }
    Ok(uuid) => {
      web::block(move || {
        dsl::cluster_networks
          .filter(dsl::id.eq(uuid))
          .get_result(&conn)
      })
      .await
    }
  };

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

#[cfg(test)]
mod cluster_networks {
  use ntex::web;

  use crate::postgre;
  use crate::repositories::cluster;
  use crate::models::ClusterPartial;

  use super::*;

  #[ntex::test]
  async fn main() {
    let pool = postgre::create_pool();
    let pool_state = web::types::State::new(pool);

    // Create cluster for relationship
    let new_cluster = ClusterPartial {
      name: String::from("dev"),
    };
    let cluster = cluster::create_for_namespace(
      String::from("default"),
      new_cluster,
      &pool_state,
    )
    .await
    .unwrap();

    // create
    let new_network = ClusterNetworkPartial {
      name: String::from("test-dev"),
    };
    let network = create_for_cluster(cluster.id, new_network, &pool_state)
      .await
      .unwrap();
    // find
    find_by_id_or_name(network.name, &pool_state).await.unwrap();

    // delete
    delete_by_id_or_name(network.id.to_string(), &pool_state)
      .await
      .unwrap();

    // clean cluster
    cluster::delete_by_gen_id("default-dev".to_string(), &pool_state)
      .await
      .unwrap();
  }
}
