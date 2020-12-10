use tide::convert::Serialize;
use tide::http::mime;
use tide::{Body, Response, StatusCode};
// use std::fmt;

#[derive(Serialize, Debug, Clone)]
pub enum ErrorType {
    PlayerNotFound,
    ParsingError,
}

#[derive(Serialize, Debug, Clone)]
pub struct CustomError {
    pub error_type: ErrorType,
    pub message: String,
}

impl Into<tide::Response> for CustomError {
    fn into(self) -> tide::Response {
        let mut response = Response::new(StatusCode::BadRequest);
        response.set_content_type(mime::JSON);
        response.set_body(Body::from_json(&self).unwrap_or(Body::from_string("{{}}".to_string())));
        response
    }
}

impl Into<tide::Result> for CustomError {
    fn into(self) -> tide::Result {
        Ok(self.into())
    }
}

pub fn argument_parsing_error(argument_name: &str, argument_type_description: &str) -> CustomError {
    CustomError {
        message: format!(
            "Could not parse argument {} into {}",
            argument_name, argument_type_description
        ),
        error_type: ErrorType::ParsingError,
    }
}
