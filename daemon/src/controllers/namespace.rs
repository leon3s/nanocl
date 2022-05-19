#[utoipa::path(
  get,
  path = "/namespaces",
  responses(
      (status = 200, description = "Array of namespace found", body = NamespaceItem),
  ),
)]
pub async fn list() {

}
