use ntex::web;
use ntex::http::StatusCode;
use serde::Serialize;
use serde_json::{json, to_string_pretty};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Serialize)]
pub struct HttpError {
    pub(crate) msg: String,
    #[serde(skip_serializing)]
    pub(crate) status: StatusCode,
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl web::WebResponseError for HttpError {
    // builds the actual response to send back when an error occurs
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let err_json = json!({ "error": self.msg });
        web::HttpResponse::build(self.status).json(&err_json)
    }
}
