use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum ErrorType {
    PlayerNotFound,
    PlayerAlreadyExisting,
    PlayerNameWrongFormat,
}

#[derive(Serialize, Debug, Clone)]
pub struct CustomError {
    pub error_type: ErrorType,
    pub message: String,
}

impl CustomError {
    pub fn new(message: String, error_type: ErrorType) -> CustomError {
        CustomError {
            message: message,
            error_type: error_type,
        }
    }
    pub fn new_player_not_found_by_name_error(name: String) -> CustomError {
        CustomError::new(
            format!("Could not find a player with name {}", name),
            ErrorType::PlayerNotFound,
        )
    }
    pub fn new_player_not_found_by_id_error(id: i32) -> CustomError {
        CustomError::new(
            format!("Could not find a player with id {}", id),
            ErrorType::PlayerNotFound,
        )
    }
    pub fn new_player_already_existing_error(name: &String) -> CustomError {
        CustomError::new(
            format!("A Player with the name {} does already exist", name),
            ErrorType::PlayerAlreadyExisting,
        )
    }
    pub fn player_name_has_wrong_format(name: &String) -> CustomError {
        CustomError::new(
            format!("The name {} has the wrong format", name),
            ErrorType::PlayerNameWrongFormat,
        )
    }
}

impl Responder for CustomError {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
