use ntex::web;
use serde::{Serialize, Deserialize};
use sysinfo::{System, SystemExt, NetworkExt};

use crate::responses::errors;

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemMemory {
  total_memory: u64,
  used_memory: u64,
  total_swap: u64,
  used_swap: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemNetwork {
  name: String,
  received: u64,
  transmitted: u64,
}

#[web::get("/system/networks")]
async fn get_system_networks(
) -> Result<web::HttpResponse, errors::HttpError> {
  /* init system library */
  let mut sys = System::new_all();
  let mut list: Vec<SystemNetwork> = Vec::new();
  /* refresh system info */
  sys.refresh_all();
  /* Make http request to get all network in docker or kind of ip addr to show them
   * Or parse the system file or networks
   * Todo get ipv4 ipv6 and others informations
   * */
  for (name, data) in sys.networks() {
    let received = data.received();
    let transmitted = data.transmitted();
    list.push(SystemNetwork {
      name: name.to_string(),
      received,
      transmitted,
    });
  }
  Ok(
    web::HttpResponse::Ok()
    .content_type("application/json")
    .json(&list)
  )
}

#[web::get("/system/memory")]
pub async fn get_system_memory(
) -> Result<web::HttpResponse, errors::HttpError> {
  let mut sys = System::new_all();
  sys.refresh_all();
  let response = SystemMemory {
    total_memory: sys.total_memory(),
    used_memory: sys.used_memory(),
    total_swap: sys.total_swap(),
    used_swap: sys.used_swap(),
  };
  Ok(
      web::HttpResponse::Ok()
      .content_type("application/json")
      .json(&response)
  )
}

pub fn ctrl_config(config: &mut web::ServiceConfig) {
  config
  .service(get_system_memory)
  .service(get_system_networks);
}

#[cfg(test)]
mod ctrl_system_tests {
  use crate::controllers::system::*;
  use ntex::http::{StatusCode};
  use ntex::web::{test, App, Error};

  #[ntex::test]
  async fn get_system_memory() -> Result<(), Error> {
    let srv = test::server(move || {
        App::new()
        .configure(ctrl_config)
    });

    let response = srv
    .get("/system/memory")
    .send()
    .await
    .unwrap();

    let status = response.status();
    assert_eq!(status, StatusCode::OK);
    Ok(())
  }

  #[ntex::test]
  async fn get_system_networks() -> Result<(), Error> {
    let srv = test::server(move || {
      App::new()
      .configure(ctrl_config)
    });

    let mut response = srv
    .get("/system/networks")
    .send()
    .await
    .unwrap();

    let body = response.body().await;

    println!("{:?}", body);
    let status = response.status();
    assert_eq!(status, StatusCode::OK);
    Ok(())
  }
}
