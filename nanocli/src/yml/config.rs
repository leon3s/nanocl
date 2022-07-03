use std::path::PathBuf;

use futures::StreamExt;
use futures::stream::FuturesUnordered;

use crate::nanocld::cargo::CargoPartial;
use crate::nanocld::client::Nanocld;
use crate::nanocld::cluster::{
  ClusterNetworkPartial, ClusterPartial, ClusterVarPartial,
};

use crate::errors::CliError;

use super::parser::get_config_type;
use super::models::{YmlConfigTypes, NamespaceConfig};

async fn delete_namespace(
  namespace: &NamespaceConfig,
  client: &Nanocld,
) -> Result<(), CliError> {
  // Delete cargoes
  namespace
    .cargoes
    .iter()
    .map(|cargo| async {
      client.delete_cargo(cargo.name.to_owned()).await?;
      Ok::<(), CliError>(())
    })
    .collect::<FuturesUnordered<_>>()
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .collect::<Result<Vec<()>, CliError>>()?;

  // Delete clusters
  namespace
    .clusters
    .iter()
    .map(|cluster| async {
      client.delete_cluster(cluster.name.to_owned()).await?;
      Ok::<(), CliError>(())
    })
    .collect::<FuturesUnordered<_>>()
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .collect::<Result<Vec<()>, CliError>>()?;

  Ok(())
}

async fn apply_namespace(
  namespace: &NamespaceConfig,
  client: &Nanocld,
) -> Result<(), CliError> {
  // Create namespace if not exists
  if client.inspect_namespace(&namespace.name).await.is_err() {
    client.create_namespace(&namespace.name).await?;
  }

  // Create clusters
  namespace
    .clusters
    .iter()
    .map(|cluster| async {
      let item = ClusterPartial {
        name: cluster.name.to_owned(),
      };
      client.create_cluster(&item).await?;
      // Create cluster variables
      if let Some(variables) = cluster.variables.to_owned() {
        let variables = &variables;
        variables
          .to_owned()
          .into_keys()
          .map(|var_name| async {
            let value = variables.get(&var_name).unwrap();

            let item = ClusterVarPartial {
              name: var_name,
              value: value.into(),
            };
            client
              .create_cluster_var(&cluster.name.to_owned(), item)
              .await?;
            Ok::<_, CliError>(())
          })
          .collect::<FuturesUnordered<_>>()
          .collect::<Vec<_>>()
          .await
          .into_iter()
          .collect::<Result<Vec<()>, CliError>>()?;
      }
      // Create cluster networks
      namespace
        .networks
        .iter()
        .map(|network| async {
          let item = ClusterNetworkPartial {
            name: network.name.to_owned(),
          };
          client
            .create_cluster_network(cluster.name.to_owned(), &item)
            .await?;

          Ok::<_, CliError>(())
        })
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<()>, CliError>>()?;
      Ok::<_, CliError>(())
    })
    .collect::<FuturesUnordered<_>>()
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .collect::<Result<Vec<()>, CliError>>()?;

  // Create cargoes
  namespace
    .cargoes
    .iter()
    .map(|cargo| async {
      let item = CargoPartial {
        name: cargo.name.to_owned(),
        image_name: cargo.image_name.to_owned(),
        proxy_config: cargo.proxy_config.to_owned(),
        environnements: cargo.environnements.to_owned(),
      };
      client.create_cargo(&item).await?;
      // client.join_cluster_cargo(&cluster.name.to_owned(), )
      Ok::<_, CliError>(())
    })
    .collect::<FuturesUnordered<_>>()
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .collect::<Result<Vec<()>, CliError>>()?;

  namespace
    .clusters
    .iter()
    .map(|cluster| async {
      if let Some(joins) = &cluster.joins {
        joins
          .iter()
          .map(|join| async {
            client.join_cluster_cargo(&cluster.name, join).await?;

            Ok::<_, CliError>(())
          })
          .collect::<FuturesUnordered<_>>()
          .collect::<Vec<_>>()
          .await
          .into_iter()
          .collect::<Result<Vec<()>, CliError>>()?;
      }

      if let Some(auto_start) = cluster.auto_start {
        if !auto_start {
          return Ok::<_, CliError>(());
        }
        client.start_cluster(&cluster.name).await?;
      }

      Ok::<_, CliError>(())
    })
    .collect::<FuturesUnordered<_>>()
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .collect::<Result<Vec<()>, CliError>>()?;

  Ok(())
}

pub async fn apply(
  file_path: PathBuf,
  client: &Nanocld,
) -> Result<(), CliError> {
  let file_content = std::fs::read_to_string(file_path)?;
  let config_type = get_config_type(&file_content)?;
  println!("config_type : {:#?}", &config_type);
  match config_type {
    YmlConfigTypes::Namespace => {
      let namespace = serde_yaml::from_str::<NamespaceConfig>(&file_content)?;
      println!("namespace config {:#?}", &namespace);
      apply_namespace(&namespace, client).await?;
    }
    _ => todo!("apply different type of config"),
  }
  Ok(())
}

pub async fn delete(
  file_path: PathBuf,
  client: &Nanocld,
) -> Result<(), CliError> {
  let file_content = std::fs::read_to_string(file_path)?;
  let config_type = get_config_type(&file_content)?;
  println!("config_type : {:#?}", &config_type);
  match config_type {
    YmlConfigTypes::Namespace => {
      let namespace = serde_yaml::from_str::<NamespaceConfig>(&file_content)?;
      println!("namespace config {:#?}", &namespace);
      delete_namespace(&namespace, client).await?;
    }
    _ => todo!("delete different type of config"),
  }
  Ok(())
}
