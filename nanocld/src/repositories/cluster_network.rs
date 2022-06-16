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
  cluster_key: String,
  item: ClusterNetworkPartial,
  pool: &web::types::State<Pool>,
) -> Result<ClusterNetworkItem, HttpError> {
  use crate::schema::cluster_networks::dsl;
  let conn = get_pool_conn(pool)?;

  let res = web::block(move || {
    let item = ClusterNetworkItem {
      key: cluster_key.to_owned() + "-" + &item.name,
      cluster_key,
      name: item.name,
      docker_network_id: item.docker_network_id,
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
          .filter(dsl::key.eq(uuid.to_string()))
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
          .filter(dsl::key.eq(uuid.to_string()))
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
      docker_network_id: id,
    };
    let network = create_for_cluster(cluster.key, new_network, &pool_state)
      .await
      .unwrap();

    let network_name = network.name.clone();
    // find cluster network
    find_by_id_or_name(network_name.clone(), &pool_state)
      .await
      .unwrap();

    // delete cluster network
    delete_by_id_or_name(network_name.clone(), &pool_state)
      .await
      .unwrap();

    // clean cluster
    cluster::delete_by_key("default-dev".to_string(), &pool_state)
      .await
      .unwrap();

    // clean docker network
    docker.remove_network(NET_NAME).await.unwrap();
  }
}
