pub fn gen_nsp_key_by_name(
  namespace: &Option<String>,
  name: &String,
) -> String {
  match namespace {
    None => format!("global-{}", name),
    Some(namespace) => format!("{}-{}", namespace, name),
  }
}
