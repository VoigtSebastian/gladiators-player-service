use crate::error::CustomError;
use actix_web::{Error, HttpRequest, HttpResponse, Responder, Result};
use futures::future::{ready, Ready};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref PLAYER_NAME_REGEX: Regex =
        Regex::new(r"^\p{letter}[\p{letter}|_|\d]{3,20}$").unwrap();
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Player {
    pub id: i32,
    pub player_name: String,
    pub games_played: i32,
    pub games_won: i32,
}

impl Responder for Player {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

/// Struct to represent a players name.
///
/// This struct should be used when passing around a players name, as it
/// assures a certain format (regex: PLAYER_NAME_REGEX).
pub struct PlayerName {
    name: String,
}

impl PlayerName {
    /// Function used to create a Player.
    ///
    /// This function uses PLAYER_NAME_REGEX to assure, that the name has the
    /// correct format.
    pub fn new(name: &String) -> Result<PlayerName, CustomError> {
        if PLAYER_NAME_REGEX.is_match(&name) {
            return Ok(PlayerName {
                name: name.to_string(),
            });
        } else {
            Err(CustomError::player_name_has_wrong_format(name))
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[test]
fn player_names() {
    assert_eq!(true, PlayerName::new(&"aaaa".to_string()).is_ok());
    assert_eq!(true, PlayerName::new(&"dave".to_string()).is_ok());
    assert_eq!(true, PlayerName::new(&"dave_dave_".to_string()).is_ok());
    assert_eq!(true, PlayerName::new(&"dave_d4ve".to_string()).is_ok());
    assert_eq!(true, PlayerName::new(&"dave_".to_string()).is_ok());
    assert_eq!(true, PlayerName::new(&"davü".to_string()).is_ok());
    assert_eq!(true, PlayerName::new(&"dave_ß".to_string()).is_ok());
    assert_eq!(true, PlayerName::new(&"da1234ve".to_string()).is_ok());

    assert_ne!(true, PlayerName::new(&"-".to_string()).is_ok());
    assert_ne!(true, PlayerName::new(&"_dave".to_string()).is_ok());
    assert_ne!(true, PlayerName::new(&"dave dave".to_string()).is_ok());
    assert_ne!(true, PlayerName::new(&" dave".to_string()).is_ok());
    assert_ne!(true, PlayerName::new(&"da#ve".to_string()).is_ok());
}
