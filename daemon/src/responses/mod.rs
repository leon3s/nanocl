use ntex::web;
use mongodb::error::Error as MongoError;

pub mod models;

// Todo generic mongo errors
pub fn mongo_error(_error: MongoError) -> web::HttpResponse {
  web::HttpResponse::InternalServerError()
  .content_type("application/json")
  .json(& models::ErrorResponse {
    message: String::from("unexpected mongodb error."),
  })
}
