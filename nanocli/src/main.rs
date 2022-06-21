use clap::Parser;

#[cfg(feature = "mangen")]
use clap_mangen::Man;
#[cfg(feature = "mangen")]
use clap_complete::generate_to;
#[cfg(feature = "mangen")]
use std::{path::Path, fs::File};
#[cfg(feature = "mangen")]
use clap::IntoApp;

use std::process::{Command, Stdio};

use tabled::{
  object::{Segment, Rows},
  Padding, Alignment, Table, Style, Modify,
};

mod cli;
mod models;
mod nanocld;

use cli::*;

use nanocld::error::Error;

fn process_error(err: Error) {
  match err {
    Error::Api(err) => {
      println!("{}", err.msg);
    }
    _ => eprintln!("{:?}", err),
    // Error::Payload(_) => todo!(),
    // Error::SendRequest(_) => todo!(),
    // Error::JsonPayload(_) => todo!(),
  }
  std::process::exit(1);
}

#[cfg(feature = "mangen")]
fn build_manpages(outdir: &Path) -> Result<(), Box<dyn std::error::Error>> {
  let app = Cli::into_app();

  let file = Path::new(&outdir).join("nanocli.1");
  let mut file = File::create(&file)?;

  Man::new(app).render(&mut file)?;

  Ok(())
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

#[ntex::main]
async fn main() -> std::io::Result<()> {
  let args = Cli::parse();
  let client = nanocld::client::Nanocld::connect_with_unix_default().await;
  #[cfg(feature = "mangen")]
  {
    let dir = std::env::current_dir()?;
    build_manpages(&dir).unwrap();
  }
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
        match client.create_namespace(item.name.to_owned()).await {
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
      CargoCommands::Start(options) => {
        if let Err(err) = client.start_cargo(options.name.to_owned()).await {
          process_error(err);
        }
      }
    },
  }
  // REAM yml file
  // let args: Vec<String> = env::args().collect();
  // if args.len() < 2 {
  //   eprintln!("require the file path to parse as argument");
  // }
  // let mut path = env::current_dir()?;
  // path.push(&args[1]);

  // let result = fs::read_to_string(&path);
  // if let Ok(str) = result {
  //   if let Err(err) = parse_config(&str) {
  //     eprintln!("error while parsing config {:?}", err);
  //   }
  // } else {
  //   eprintln!("the file {:?} cannot be parsed", path);
  // }
  Ok(())
}
