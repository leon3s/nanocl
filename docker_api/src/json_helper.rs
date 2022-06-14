pub(crate) fn deserialize_nonoptional_vec<
  'de,
  D: serde::Deserializer<'de>,
  T: serde::de::DeserializeOwned,
>(
  d: D,
) -> Result<Vec<T>, D::Error> {
  serde::Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or_default())
}

pub(crate) fn deserialize_nonoptional_map<
  'de,
  D: serde::Deserializer<'de>,
  T: serde::de::DeserializeOwned,
>(
  d: D,
) -> Result<std::collections::HashMap<String, T>, D::Error> {
  serde::Deserialize::deserialize(d)
    .map(|x: Option<_>| x.unwrap_or(std::collections::HashMap::new()))
}
