use ntex::web;

use super::http_error;

#[web::get("/namespaces/{name}/clusters")]
async fn list() -> Result<web::HttpResponse, http_error::HttpError> {

  


  Ok(web::HttpResponse::Ok().body("gg"))
}

pub fn config_ntex(config: &mut web::ServiceConfig) {

}