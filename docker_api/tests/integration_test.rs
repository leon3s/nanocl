#[ntex::test]
async fn init_api() {
  let _docker = docker_api::Api::new();
}
