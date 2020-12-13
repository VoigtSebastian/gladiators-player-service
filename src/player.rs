use tide::convert::{Deserialize, Serialize};
use tide::http::mime;
use tide::{Body, Response, StatusCode};
use crate::error::CustomError;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PLAYER_NAME_REGEX: Regex = Regex::new(r"^\p{letter}[\p{letter}|_|\d]{3,20}$").unwrap();
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Player {
    pub id: i32,
    pub player_name: String,
    pub games_played: i32,
    pub games_won: i32,
}

pub struct PlayerName {
    pub name: String
}

impl PlayerName {
    pub fn new(name: &String) -> Result<PlayerName, CustomError> {
        if PLAYER_NAME_REGEX.is_match(&name) {
            return Ok(PlayerName { name: name.to_string() });
        } else {
            Err(CustomError::player_name_has_wrong_format(name))
        }
    }
}

impl Player {
    pub fn new(id: i32, player_name: String, games_played: i32, games_won: i32) -> Player {
        Player {
            id: id,
            player_name: player_name,
            games_played: games_played,
            games_won: games_won,
        }
    }
    pub fn build_response(&self) -> tide::Result {
        let mut response = Response::new(StatusCode::Ok);
        response.set_content_type(mime::JSON);
        response.set_body(Body::from_json(&self)?);
        Ok(response)
    }
}

/// Builds a tide::Result from a vector of Players.
///
/// The Response has the status-code 200 and contains a json serialization
/// of all Players contained in the parameter.
pub fn player_vector_response(players: &Vec<Player>) -> tide::Result {
    let mut response = Response::new(StatusCode::Ok);
    response.set_content_type(mime::JSON);
    response.set_body(Body::from_json(&players)?);
    Ok(response)
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
