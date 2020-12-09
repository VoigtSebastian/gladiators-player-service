use tide::convert::{Deserialize, Serialize};
use tide::http::mime;
use tide::{Body, Response, StatusCode};

#[derive(Serialize)]
pub enum ErrorType {
    PlayerNotFound,
}

#[derive(Serialize)]
pub struct CustomError {
    pub error_type: ErrorType,
    pub message: String,
}

impl CustomError {
    pub fn build_response(&self) -> tide::Result {
        let mut response = Response::new(StatusCode::BadRequest);
        response.set_content_type(mime::JSON);
        response.set_body(Body::from_json(self)?);
        Ok(response)
    }
}
