use crate::error::{argument_parsing_error, CustomError, ErrorType};
use crate::player::query_player;
use crate::player::{player_vector_response, Player};
use crate::state::State;

// curl localhost:8080/echo/player \
// -d '{ "id": 1, "player_name": "test", "games_played": 2, "games_won": 3}'
pub async fn echo_player(mut req: tide::Request<State>) -> tide::Result {
    let gladiator: Player = req.body_json().await?;
    gladiator.build_response()
}

// curl localhost:8080/echo/players \
// -d '[{ "id": 1, "player_name": "test", "games_played": 2, "games_won": 3}, { "id": 2, "name": "test", "games_played": 2, "games_won": 3}]'
pub async fn echo_players(mut req: tide::Request<State>) -> tide::Result {
    let gladiators: Vec<Player> = req.body_json().await?;
    player_vector_response(&gladiators)
}

// curl localhost:8080/player/4
pub async fn player_lookup(req: tide::Request<State>) -> tide::Result {
    let id = match req.param("id")?.parse::<i32>() {
        Err(_) => return argument_parsing_error("id", "i32").into(),
        Ok(id) => id,
    };

    match query_player(&req.state().pg_pool, id).await {
        Some(gladiator) => gladiator.build_response(),
        None => CustomError {
            error_type: ErrorType::PlayerNotFound,
            message: format!("Could not find a player with id {}", id),
        }
        .into(),
    }
}
