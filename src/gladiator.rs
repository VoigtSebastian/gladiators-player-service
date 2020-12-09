use tide::convert::{Deserialize, Serialize};
use tide::http::mime;
use tide::{Body, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize)]
pub struct Gladiator {
    id: u32,
    player_name: String,
    games_played: u32,
    games_won: u32,
}

impl Gladiator {
    pub fn build_response(&self) -> tide::Result {
        let mut response = Response::new(StatusCode::Ok);
        response.set_content_type(mime::JSON);
        response.set_body(Body::from_json(&self)?);
        Ok(response)
    }
}

pub fn gladiator_vector_response(gladiators: &Vec<Gladiator>) -> tide::Result {
    let mut response = Response::new(StatusCode::Ok);
    response.set_content_type(mime::JSON);
    response.set_body(Body::from_json(&gladiators)?);
    Ok(response)
}
