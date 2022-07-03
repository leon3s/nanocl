use clap::IntoApp;
use crate::cli::Cli;
use crate::cli::NamespaceArgs;

pub fn generate_man() -> std::io::Result<()> {
  let man = clap_mangen::Man::new(Cli::into_app());
  let man_namespace = clap_mangen::Man::new(NamespaceArgs::into_app());
  let mut man_buffer: Vec<u8> = Default::default();
  man.render(&mut man_buffer)?;
  let mut man_namespace_buffer: Vec<u8> = Default::default();
  man_namespace.render(&mut man_namespace_buffer)?;

  let out_dir = std::env::current_dir()?;
  std::fs::write(out_dir.join("nanocl.1"), man_buffer)?;
  std::fs::write(out_dir.join("nanocl-namespace.1"), man_namespace_buffer)?;

  Ok(())
}
