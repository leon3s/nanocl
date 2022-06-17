use ntex::web;
use diesel::prelude::*;

use crate::utils::get_pool_conn;
use crate::controllers::errors::HttpError;
use crate::models::{
  Pool, ClusterNetworkPartial, ClusterNetworkItem, PgDeleteGeneric, ClusterItem,
};

use super::errors::db_blocking_error;

// Vec<ClusterNetworkItem>
pub async fn list_for_cluster(
  cluster: ClusterItem,
  pool: &web::types::State<Pool>,
) -> Result<Vec<ClusterNetworkItem>, HttpError> {
  let conn = get_pool_conn(pool)?;
  let res = web::block(move || {
    ClusterNetworkItem::belonging_to(&cluster).load::<ClusterNetworkItem>(&conn)
  })
  .await;
  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(items) => Ok(items),
  }
}

pub async fn create_for_cluster(
  cluster_key: String,
  item: ClusterNetworkPartial,
  docker_network_id: String,
  pool: &web::types::State<Pool>,
) -> Result<ClusterNetworkItem, HttpError> {
  use crate::schema::cluster_networks::dsl;
  let conn = get_pool_conn(pool)?;

  let res = web::block(move || {
    let item = ClusterNetworkItem {
      key: cluster_key.to_owned() + "-" + &item.name,
      cluster_key,
      name: item.name,
      docker_network_id,
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

pub async fn delete_by_key(
  key: String,
  pool: &web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
  use crate::schema::cluster_networks::dsl;
  let conn = get_pool_conn(pool)?;

  let res = web::block(move || {
    diesel::delete(dsl::cluster_networks)
      .filter(dsl::key.eq(key))
      .execute(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(result) => Ok(PgDeleteGeneric { count: result }),
  }
}

pub async fn find_by_key(
  key: String,
  pool: &web::types::State<Pool>,
) -> Result<ClusterNetworkItem, HttpError> {
  use crate::schema::cluster_networks::dsl;
  let conn = get_pool_conn(pool)?;

  let res = web::block(move || {
    dsl::cluster_networks
      .filter(dsl::key.eq(key))
      .get_result(&conn)
  })
  .await;

  match res {
    Err(err) => Err(db_blocking_error(err)),
    Ok(item) => Ok(item),
  }
}

#[cfg(test)]
mod cluster_networks {
  use ntex::web;
  use bollard::network::CreateNetworkOptions;

  use crate::postgre;
  use crate::repositories::cluster;
  use crate::models::ClusterPartial;

  use super::*;

  #[ntex::test]
  async fn main() {
    const NET_NAME: &str = "test-cluster-network";

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

    // create docker network for relationship
    let docker = bollard::Docker::connect_with_local_defaults().unwrap();
    let net_config = CreateNetworkOptions {
      name: NET_NAME,
      ..Default::default()
    };
    let network = docker.create_network(net_config).await.unwrap();

    let id = match network.id {
      None => panic!("unable to bind network id"),
      Some(id) => id,
    };

    // create cluster network
    let new_network = ClusterNetworkPartial {
      name: String::from("test-dev"),
    };
    let network = create_for_cluster(cluster.key, new_network, id, &pool_state)
      .await
      .unwrap();

    let n_key = network.key.clone();
    // find cluster network
    find_by_key(n_key.clone(), &pool_state).await.unwrap();

    // delete cluster network
    delete_by_key(n_key.clone(), &pool_state).await.unwrap();

    // clean cluster
    cluster::delete_by_key("default-dev".to_string(), &pool_state)
      .await
      .unwrap();

    // clean docker network
    docker.remove_network(NET_NAME).await.unwrap();
  }
}
