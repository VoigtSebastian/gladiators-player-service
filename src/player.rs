use sqlx::postgres::PgPool;
use tide::convert::{Deserialize, Serialize};
use tide::http::mime;
use tide::{Body, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Player {
    id: i32,
    player_name: String,
    games_played: i32,
    games_won: i32,
}

impl Player {
    pub fn build_response(&self) -> tide::Result {
        let mut response = Response::new(StatusCode::Ok);
        response.set_content_type(mime::JSON);
        response.set_body(Body::from_json(&self)?);
        Ok(response)
    }
}

pub async fn query_player(connection: &PgPool, id: i32) -> Option<Player> {
    sqlx::query_as::<_, Player>("SELECT * FROM players WHERE id = $1;")
        .bind(id)
        .fetch_optional(connection)
        .await
        .unwrap_or(None)
}

pub fn player_vector_response(player: &Vec<Player>) -> tide::Result {
    let mut response = Response::new(StatusCode::Ok);
    response.set_content_type(mime::JSON);
    response.set_body(Body::from_json(&player)?);
    Ok(response)
}