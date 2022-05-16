use self::mongo::{models, repository::Repository};

pub mod mongo;

#[derive(Debug, Clone)]
pub struct Repositories {
  pub(crate) namespace: Repository<models::Namespace>,
}
