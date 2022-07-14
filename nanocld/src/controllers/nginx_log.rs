use ntex::{web, rt};
use ntex::{channel::mpsc::channel, util::Bytes};
use futures::{
  SinkExt, StreamExt,
  channel::mpsc::{unbounded, UnboundedSender},
};

use crate::events::system::EventSystemClient;
use crate::{errors::HttpResponseError, events::system::EventMessage};

#[web::get("nginx/logs")]
async fn stream_nginx_logs(
  system_event: web::types::State<UnboundedSender<EventMessage>>,
) -> Result<web::HttpResponse, HttpResponseError> {
  let mut system_event = system_event.as_ref();
  let (tx, mut rx) = unbounded::<Bytes>();
  let (tx_body, rx_body) = channel::<Result<Bytes, web::error::Error>>();

  let client = EventSystemClient::new(tx);
  let _res = system_event.send(EventMessage::NginxLog(client)).await;
  rt::spawn(async move {
    while let Some(msg) = rx.next().await {
      if let Err(err) = tx_body.send(Ok::<_, web::error::Error>(msg)) {
        log::error!("{}", err);
        break;
      }
    }
  });
  Ok(
    web::HttpResponse::Ok()
      .keep_alive()
      .content_type("nanocl/streaming-v1")
      .streaming(rx_body),
  )
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
  config.service(stream_nginx_logs);
}
