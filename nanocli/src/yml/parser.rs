fn _parse_config(str: &str) -> Result<models::YmlFile, serde_yaml::Error> {
  let result: models::YmlFile = serde_yaml::from_str(str)?;
  Ok(result)
}
