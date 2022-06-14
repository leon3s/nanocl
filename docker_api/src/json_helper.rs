pub(crate) fn deserialize_nonoptional_vec<
  'de,
  D: serde::Deserializer<'de>,
  T: serde::de::DeserializeOwned,
>(
  d: D,
) -> Result<Vec<T>, D::Error> {
  serde::Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or_default())
}

pub(crate) fn _deserialize_nonoptional_map<
  'de,
  D: serde::Deserializer<'de>,
  T: serde::de::DeserializeOwned,
>(
  d: D,
) -> Result<std::collections::HashMap<String, T>, D::Error> {
  serde::Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or_default())
}

pub(crate) fn serialize_as_json<T, S>(t: &T, s: S) -> Result<S::Ok, S::Error>
where
  T: serde::Serialize,
  S: serde::Serializer,
{
  s.serialize_str(
    &serde_json::to_string(t)
      .map_err(|e| serde::ser::Error::custom(format!("{}", e)))?,
  )
}
