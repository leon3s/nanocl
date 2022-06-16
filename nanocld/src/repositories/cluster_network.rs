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
      docker_network_id: item.docker_network_id,
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
    let network = create_for_cluster(cluster.id, new_network, &pool_state)
      .await
      .unwrap();

    // find cluster network
    find_by_id_or_name(network.name, &pool_state).await.unwrap();

    // delete cluster network
    delete_by_id_or_name(network.id.to_string(), &pool_state)
      .await
      .unwrap();

    // clean cluster
    cluster::delete_by_gen_id("default-dev".to_string(), &pool_state)
      .await
      .unwrap();

    // clean docker network
    docker.remove_network(NET_NAME).await.unwrap();
  }
}
