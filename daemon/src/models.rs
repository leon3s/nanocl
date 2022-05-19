use utoipa::Component;

#[derive(Component)]
pub struct NamespaceItem {
  id: String,
  name: String,
}
