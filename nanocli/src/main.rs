use clap::Parser;
use errors::CliError;
use futures::StreamExt;
use futures::stream::FuturesUnordered;
use serde::{Serialize, Deserialize};

use std::process::{Command, Stdio};

use tabled::{
  object::{Segment, Rows},
  Padding, Alignment, Table, Style, Modify, Tabled,
};

mod cli;
mod yml;
mod errors;
mod nanocld;
#[cfg(feature = "genman")]
mod man;

use cli::*;

fn process_error(err: errors::CliError) {
  eprintln!("{}", err);
  std::process::exit(1);
}

fn print_table<T>(iter: impl IntoIterator<Item = T>)
where
  T: tabled::Tabled,
{
  let table = Table::new(iter)
    .with(Style::empty())
    .with(
      Modify::new(Segment::all())
        .with(Padding::new(0, 4, 0, 0))
        .with(Alignment::left()),
    )
    .with(Modify::new(Rows::first()).with(str::to_uppercase))
    .to_string();
  print!("{}", table);
}

#[derive(Debug, Tabled, Serialize, Deserialize)]
pub struct NamespaceWithCount {
  name: String,
  cargoes: usize,
  clusters: usize,
  networks: usize,
}

async fn execute_args(args: Cli) -> Result<(), CliError> {
  let client = nanocld::client::Nanocld::connect_with_unix_default().await;
  match &args.command {
    Commands::Docker(options) => {
      let mut opts = vec![
        String::from("-H"),
        String::from("unix:///run/nanocl/docker.sock"),
      ];
      let mut more_options = options.args.clone();
      opts.append(&mut more_options);

      let mut cmd = Command::new("docker")
        .args(&opts)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

      let _status = cmd.wait();
    }
    Commands::Namespace(args) => match &args.commands {
      NamespaceCommands::List => {
        let items = client.list_namespace().await?;
        let namespaces = items
          .iter()
          .map(|item| async {
            let cargo_count = client.count_cargo(&item.name).await?;
            let cluster_count = client.count_cluster(&item.name).await?;
            let network_count =
              client.count_cluster_network_by_nsp(&item.name).await?;
            let new_item = NamespaceWithCount {
              name: item.name.to_owned(),
              cargoes: cargo_count.count,
              clusters: cluster_count.count,
              networks: network_count.count,
            };
            Ok::<_, CliError>(new_item)
          })
          .collect::<FuturesUnordered<_>>()
          .collect::<Vec<_>>()
          .await
          .into_iter()
          .collect::<Result<Vec<NamespaceWithCount>, CliError>>()?;

        print_table(namespaces);
      }
      NamespaceCommands::Create(item) => {
        let item = client.create_namespace(&item.name).await?;
        println!("{}", item.name);
      }
    },
    Commands::Cluster(args) => match &args.commands {
      ClusterCommands::List => {
        let items = client.list_cluster().await?;
        print_table(items);
      }
      ClusterCommands::Create(item) => {
        let item = client.create_cluster(item).await?;
        println!("{}", item.key);
      }
      ClusterCommands::Remove(options) => {
        client.delete_cluster(options.name.to_owned()).await?;
      }
      ClusterCommands::Start(options) => {
        client.start_cluster(&options.name).await?;
      }
    },
    Commands::ClusterNetwork(args) => match &args.commands {
      ClusterNetworkCommands::List => {
        let items =
          client.list_cluster_network(args.cluster.to_owned()).await?;
        print_table(items);
      }
      ClusterNetworkCommands::Create(item) => {
        let item = client
          .create_cluster_network(args.cluster.to_owned(), item)
          .await?;
        println!("{}", item.key);
      }
      ClusterNetworkCommands::Remove(options) => {
        client
          .delete_cluster_network(
            args.cluster.to_owned(),
            options.name.to_owned(),
          )
          .await?;
      }
    },
    Commands::GitRepository(args) => match &args.commands {
      GitRepositoryCommands::List => {
        let items = client.list_git_repository().await?;
        print_table(items);
      }
      GitRepositoryCommands::Create(item) => {
        client.create_git_repository(item).await?;
        println!("{}", item.name);
      }
      GitRepositoryCommands::Remove(options) => {
        client
          .delete_git_repository(options.name.to_owned())
          .await?;
      }
      GitRepositoryCommands::Build(options) => {
        client.build_git_repository(options.name.to_owned()).await?;
      }
    },
    Commands::Cargo(args) => match &args.commands {
      CargoCommands::List => {
        let items = client.list_cargo().await?;
        print_table(items);
      }
      CargoCommands::Create(item) => {
        let item = client.create_cargo(item).await?;
        println!("{}", item.key);
      }
      CargoCommands::Remove(options) => {
        client.delete_cargo(options.name.to_owned()).await?;
      }
    },
    Commands::Apply(args) => {
      let mut file_path = std::env::current_dir()?;
      file_path.push(&args.file_path);
      println!("apply !");
      yml::config::apply(file_path, &client).await?;
    }
    Commands::Delete(args) => {
      let mut file_path = std::env::current_dir()?;
      file_path.push(&args.file_path);
      println!("delete !");
      yml::config::delete(file_path, &client).await?;
    }
  }
  Ok(())
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
  #[cfg(feature = "genman")]
  {
    man::generate_man()?;
    std::process::exit(0);
  }
  let args = Cli::parse();
  if let Err(err) = execute_args(args).await {
    process_error(err);
  }
  Ok(())
}
