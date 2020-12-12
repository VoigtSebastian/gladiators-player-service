use tide::convert::Serialize;
use tide::http::mime;
use tide::{Body, Response, StatusCode};

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

impl CustomError {
    pub fn new(message: String, error_type: ErrorType) -> CustomError {
        CustomError {
            message: message,
            error_type: error_type,
        }
    }
    pub fn new_argument_parsing_error(
        argument_name: &str,
        argument_type_description: &str,
    ) -> CustomError {
        CustomError::new(
            format!(
                "Could not parse argument {} into {}",
                argument_name, argument_type_description
            ),
            ErrorType::ParsingError,
        )
    }
    pub fn new_player_not_found_by_name_error(name: String) -> CustomError {
        CustomError::new(
            format!("Could not find a player with name {}", name),
            ErrorType::PlayerNotFound,
        )
    }
    pub fn new_player_not_found_by_id_error(id: i32) -> CustomError {
        CustomError::new(
            format!("Could not find a player with name {}", id),
            ErrorType::PlayerNotFound,
        )
    }
}
