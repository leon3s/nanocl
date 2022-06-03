use bollard::container::StatsOptions;
use futures::TryStreamExt;
use ntex::{http::StatusCode, web};

use super::http_error::HttpError;
use crate::models::Docker;

#[web::get("/containers/{id}/stats")]
pub async fn container_stats(
    id_or_name: web::types::Path<String>,
    docker: web::types::State<Docker>,
) -> Result<web::HttpResponse, HttpError> {
    let id = id_or_name.into_inner();

    let options = Some(StatsOptions {
        stream: false,
        one_shot: false,
    });

    let mut stream = docker.stats(&id, options);

    let resp = stream.try_next().await;
    match resp {
        Err(err) => {
            eprintln!("got error {:?}", err);
            Err(HttpError {
                msg: String::from("docker error."),
                status: StatusCode::INTERNAL_SERVER_ERROR,
            })
        }
        Ok(stats) => Ok(web::HttpResponse::Ok().json(&stats)),
    }
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
    config.service(container_stats);
}
