use crate::error::{argument_parsing_error, CustomError, ErrorType};
use crate::player::{player_played_round, query_player_by_id, query_player_by_name, query_players};
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

// curl localhost:8080/player/id/:id
pub async fn player_lookup_by_id(req: tide::Request<State>) -> tide::Result {
    let id = match req.param("id")?.parse::<i32>() {
        Err(_) => return argument_parsing_error("id", "i32").into(),
        Ok(id) => id,
    };

    match query_player_by_id(&req.state().pg_pool, id).await {
        Some(player) => player.build_response(),
        None => CustomError {
            error_type: ErrorType::PlayerNotFound,
            message: format!("Could not find a player with id {}", id),
        }
        .into(),
    }
}

// TODO: Make spaces in names forbidden?
// curl localhost:8080/player/name/:name
pub async fn player_lookup_by_name(req: tide::Request<State>) -> tide::Result {
    let name = match req.param("name")?.parse::<String>() {
        Err(_) => return argument_parsing_error("name", "String").into(),
        Ok(name) => name.replace("%20", " "),
    };

    match query_player_by_name(&req.state().pg_pool, &name).await {
        Some(player) => player.build_response(),
        None => CustomError {
            error_type: ErrorType::PlayerNotFound,
            message: format!("Could not find a player with name {}", name),
        }
        .into(),
    }
}

// curl localhost:8080/players
pub async fn players_lookup(req: tide::Request<State>) -> tide::Result {
    player_vector_response(&query_players(&req.state().pg_pool).await)
}

pub async fn player_played(req: tide::Request<State>) -> tide::Result {
    let name = match req.param("name")?.parse::<String>() {
        Err(_) => return argument_parsing_error("name", "String").into(),
        Ok(name) => name.replace("%20", " "),
    };

    match player_played_round(&req.state().pg_pool, &name, false).await {
        Ok(player) => player.build_response(),
        Err(_) => CustomError {
            error_type: ErrorType::PlayerNotFound,
            message: format!("Could not find a player with name {}", name),
        }
        .into(),
    }
}

pub async fn player_won(req: tide::Request<State>) -> tide::Result {
    let name = match req.param("name")?.parse::<String>() {
        Err(_) => return argument_parsing_error("name", "String").into(),
        Ok(name) => name.replace("%20", " "),
    };

    match player_played_round(&req.state().pg_pool, &name, true).await {
        Ok(player) => player.build_response(),
        Err(_) => CustomError {
            error_type: ErrorType::PlayerNotFound,
            message: format!("Could not find a player with name {}", name),
        }
        .into(),
    }
}
