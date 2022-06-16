#[cfg(test)]
pub mod simple_scenario {

  async fn before_test() {}

  #[ntex::test]
  async fn scenario() {
    before_test().await;
  }
}
