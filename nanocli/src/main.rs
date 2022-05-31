use std::{fs, env};

mod models;

fn parse_config(str: &str) -> Result<models::YmlFile, serde_yaml::Error> {
  let result: models::YmlFile = serde_yaml::from_str(str)?;
  Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("require the file path to parse as argument");
  }
  let mut path = env::current_dir()?;
  path.push(&args[1]);

  let result = fs::read_to_string(&path);
  if let Ok(str) = result {
    if let Err(err) = parse_config(&str) {
      eprintln!("error while parsing config {:?}", err);
    }
  } else {
    eprintln!("the file {:?} cannot be parsed", path);
  }
  Ok(())
}
