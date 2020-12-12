use tide::convert::{Deserialize, Serialize};
use tide::http::mime;
use tide::{Body, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Player {
    pub id: i32,
    pub player_name: String,
    pub games_played: i32,
    pub games_won: i32,
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
