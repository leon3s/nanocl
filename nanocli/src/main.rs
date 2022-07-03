use clap::Parser;

use std::process::{Command, Stdio};

use tabled::{
  object::{Segment, Rows},
  Padding, Alignment, Table, Style, Modify,
};

mod cli;
mod yml;
mod error;
mod nanocld;

use cli::*;

use nanocld::error::NanocldError;

fn process_error(err: NanocldError) {
  match err {
    NanocldError::Api(err) => {
      println!("{}", err.msg);
    }
    NanocldError::JsonPayload(err) => {
      eprintln!("{:?}", err);
    }
    _ => eprintln!("{:?}", err),
  }
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

async fn execute_args(args: Cli) {}

#[ntex::main]
async fn main() -> std::io::Result<()> {
  let args = Cli::parse();
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
      NamespaceCommands::List => match client.list_namespace().await {
        Err(err) => process_error(err),
        Ok(items) => print_table(items),
      },
      NamespaceCommands::Create(item) => {
        match client.create_namespace(&item.name).await {
          Err(err) => process_error(err),
          Ok(item) => println!("{}", item.name),
        }
      }
    },
    Commands::Cluster(args) => match &args.commands {
      ClusterCommands::List => match client.list_cluster().await {
        Err(err) => process_error(err),
        Ok(items) => print_table(items),
      },
      ClusterCommands::Create(item) => {
        match client.create_cluster(item).await {
          Err(err) => process_error(err),
          Ok(item) => println!("{}", item.key),
        }
      }
      ClusterCommands::Remove(options) => {
        if let Err(err) = client.delete_cluster(options.name.to_owned()).await {
          process_error(err);
        }
      }
      ClusterCommands::Start(options) => {
        if let Err(err) = client.start_cluster(&options.name).await {
          process_error(err);
        }
      }
    },
    Commands::ClusterNetwork(args) => match &args.commands {
      ClusterNetworkCommands::List => {
        match client.list_cluster_network(args.cluster.to_owned()).await {
          Err(err) => process_error(err),
          Ok(items) => print_table(items),
        }
      }
      ClusterNetworkCommands::Create(item) => {
        match client
          .create_cluster_network(args.cluster.to_owned(), item)
          .await
        {
          Err(err) => process_error(err),
          Ok(items) => println!("{}", items.key),
        }
      }
      ClusterNetworkCommands::Remove(options) => {
        if let Err(err) = client
          .delete_cluster_network(
            args.cluster.to_owned(),
            options.name.to_owned(),
          )
          .await
        {
          process_error(err);
        }
      }
    },
    Commands::GitRepository(args) => match &args.commands {
      GitRepositoryCommands::List => match client.list_git_repository().await {
        Err(err) => process_error(err),
        Ok(items) => print_table(items),
      },
      GitRepositoryCommands::Create(item) => {
        match client.create_git_repository(item).await {
          Err(err) => process_error(err),
          Ok(item) => println!("{}", item.name),
        }
      }
      GitRepositoryCommands::Remove(options) => {
        if let Err(err) =
          client.delete_git_repository(options.name.to_owned()).await
        {
          process_error(err);
        }
      }
      GitRepositoryCommands::Build(options) => {
        if let Err(err) =
          client.build_git_repository(options.name.to_owned()).await
        {
          process_error(err);
        }
      }
    },
    Commands::Cargo(args) => match &args.commands {
      CargoCommands::List => match client.list_cargo().await {
        Err(err) => process_error(err),
        Ok(items) => print_table(items),
      },
      CargoCommands::Create(item) => match client.create_cargo(item).await {
        Err(err) => process_error(err),
        Ok(item) => println!("{}", item.key),
      },
      CargoCommands::Remove(options) => {
        if let Err(err) = client.delete_cargo(options.name.to_owned()).await {
          process_error(err);
        }
      }
    },
    Commands::Apply(args) => {
      let mut file_path = std::env::current_dir()?;
      file_path.push(&args.file_path);
      println!("apply !");
      yml::config::apply(file_path, &client).await.unwrap();
    }
    Commands::Delete(args) => {
      let mut file_path = std::env::current_dir()?;
      file_path.push(&args.file_path);
      println!("delete !");
      yml::config::delete(file_path, &client).await.unwrap();
    }
  }
  Ok(())
}
