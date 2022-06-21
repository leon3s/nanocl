use clap::{App, AppSettings, Parser, Subcommand};
use clap_complete::{generate, Generator};
use std::io;

use crate::nanocld::{
  git_repository::GitRepositoryPartial,
  namespace::NamespacePartial,
  cluster::{ClusterPartial, ClusterNetworkPartial},
  cargo::CargoPartial,
};

/// A self-sufficient vms and containers manager
#[derive(Debug, Parser)]
#[clap(
  about,
  version,
  global_setting = AppSettings::DeriveDisplayOrder,
)]
pub struct Cli {
  /// commands
  #[clap(subcommand)]
  pub command: Commands,
}

/// Namespace commands
#[derive(Debug, Subcommand)]
pub enum NamespaceCommands {
  /// create namespace
  Create(NamespacePartial),
  /// list namespaces
  #[clap(alias("ls"))]
  List,
}

#[derive(Debug, Parser)]
pub struct GitRepositoryDeleteOptions {
  pub name: String,
}

#[derive(Debug, Parser)]
pub struct ClusterDeleteOptions {
  pub name: String,
}

#[derive(Debug, Parser)]
pub struct GitRepositoryBuildOptions {
  pub name: String,
}

#[derive(Debug, Subcommand)]
pub enum GitRepositoryCommands {
  /// list git repository
  #[clap(alias("ls"))]
  List,
  /// create git repository
  Create(GitRepositoryPartial),
  /// remove git repository
  #[clap(alias("rm"))]
  Remove(GitRepositoryDeleteOptions),
  /// build a image from git repository
  Build(GitRepositoryBuildOptions),
}

#[derive(Debug, Subcommand)]
pub enum ClusterCommands {
  /// list cluster
  #[clap(alias("ls"))]
  List,
  /// create cluster
  Create(ClusterPartial),
  /// remove cluster
  #[clap(alias("rm"))]
  Remove(ClusterDeleteOptions),
}

#[derive(Debug, Parser)]
pub struct ClusterNetworkDeleteOptions {
  #[clap(long)]
  pub cluster_name: String,
  pub name: String,
}

#[derive(Debug, Parser)]
pub struct ClusterNetworkOptions {
  #[clap(long)]
  pub cluster_name: String,
}

#[derive(Debug, Subcommand)]
pub enum ClusterNetworkCommands {
  /// list cluster network
  #[clap(alias("ls"))]
  List,
  /// create cluster network
  Create(ClusterNetworkPartial),
  /// remove cluster network
  #[clap(alias("rm"))]
  Remove(ClusterNetworkDeleteOptions),
}

#[derive(Debug, Parser)]
pub struct CargoDeleteOptions {
  pub name: String,
}

#[derive(Debug, Parser)]
pub struct CargoStartOptions {
  pub name: String,
}

#[derive(Debug, Subcommand)]
#[clap(
  about,
  version,
  global_setting = AppSettings::DeriveDisplayOrder,
)]
pub enum CargoCommands {
  /// List existing cargo
  #[clap(alias("ls"))]
  List,
  /// Create a new cargo
  Create(CargoPartial),
  /// Remove cargo by it's name
  #[clap(alias("rm"))]
  Remove(CargoDeleteOptions),
  /// Start cargo by it's name
  Start(CargoStartOptions),
}

/// manage cargos
#[derive(Debug, Parser)]
#[clap(name = "nanocl cargo")]
pub struct CargoArgs {
  /// namespace to target by default global is used
  #[clap(long)]
  pub namespace: Option<String>,
  #[clap(subcommand)]
  pub commands: CargoCommands,
}

/// alias to self-managed dockerd
#[derive(Debug, Parser)]
pub struct DockerOptions {
  #[clap(multiple = true, raw = true)]
  pub args: Vec<String>,
}

/// manage namespaces
#[derive(Debug, Parser)]
pub struct NamespaceArgs {
  #[clap(subcommand)]
  pub commands: NamespaceCommands,
}

/// manage git repositories
#[derive(Debug, Parser)]
pub struct GitRepositoryArgs {
  /// namespace to target by default global is used
  #[clap(long)]
  pub namespace: Option<String>,
  #[clap(subcommand)]
  pub commands: GitRepositoryCommands,
}

/// manage clusters
#[derive(Debug, Parser)]
pub struct ClusterArgs {
  /// namespace to target by default global is used
  #[clap(long)]
  pub namespace: Option<String>,
  #[clap(subcommand)]
  pub commands: ClusterCommands,
}

/// manage cluster networks
#[derive(Debug, Parser)]
pub struct ClusterNetworkArgs {
  /// namespace to target by default global is used
  #[clap(long)]
  pub namespace: Option<String>,
  /// cluster to target
  #[clap(long)]
  pub cluster: String,
  #[clap(subcommand)]
  pub commands: ClusterNetworkCommands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
  Docker(DockerOptions),
  Namespace(NamespaceArgs),
  Cluster(ClusterArgs),
  ClusterNetwork(ClusterNetworkArgs),
  Cargo(CargoArgs),
  GitRepository(GitRepositoryArgs),
  // TODO Completion
  // Completion {
  //   /// Shell to generate completion for
  //   #[clap(arg_enum)]
  //   shell: Shell,
  // },
}

// TODO
pub fn _print_completion<G>(gen: G, app: &mut App)
where
  G: Generator,
{
  generate(gen, app, app.get_name().to_string(), &mut io::stdout());
}
