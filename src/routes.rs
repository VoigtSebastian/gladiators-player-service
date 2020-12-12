use crate::error::CustomError;
use crate::player::{player_vector_response, Player};
use crate::queries::*;
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
        Err(_) => return CustomError::new_argument_parsing_error("id", "i32").into(),
        Ok(id) => id,
    };

    match query_player_by_id(&req.state().pg_pool, id).await {
        Some(player) => player.build_response(),
        None => CustomError::new_player_not_found_by_id_error(id).into(),
    }
}

// curl localhost:8080/player/name/:name
pub async fn player_lookup_by_name(req: tide::Request<State>) -> tide::Result {
    let name = match req.param("name")?.parse::<String>() {
        Err(_) => return CustomError::new_argument_parsing_error("name", "String").into(),
        Ok(name) => name.replace("%20", " "),
    };

    match query_player_by_name(&req.state().pg_pool, &name).await {
        Some(player) => player.build_response(),
        None => CustomError::new_player_not_found_by_name_error(name).into(),
    }
}

// curl localhost:8080/players
pub async fn players_lookup(req: tide::Request<State>) -> tide::Result {
    player_vector_response(&query_players(&req.state().pg_pool).await)
}

pub async fn player_played(req: tide::Request<State>) -> tide::Result {
    let name = match req.param("name")?.parse::<String>() {
        Err(_) => return CustomError::new_argument_parsing_error("name", "String").into(),
        Ok(name) => name.replace("%20", " "),
    };

    match player_played_round(&req.state().pg_pool, &name, false).await {
        Ok(player) => player.build_response(),
        Err(_) => CustomError::new_player_not_found_by_name_error(name).into(),
    }
}

pub async fn player_won(req: tide::Request<State>) -> tide::Result {
    let name = match req.param("name")?.parse::<String>() {
        Err(_) => return CustomError::new_argument_parsing_error("name", "String").into(),
        Ok(name) => name.replace("%20", " "),
    };

    match player_played_round(&req.state().pg_pool, &name, true).await {
        Ok(player) => player.build_response(),
        Err(_) => CustomError::new_player_not_found_by_name_error(name).into(),
    }
}

pub async fn register_player(req: tide::Request<State>) -> tide::Result {
    let name = match req.param("name")?.parse::<String>() {
        Err(_) => return CustomError::new_argument_parsing_error("name", "String").into(),
        Ok(name) => name.replace("%20", " "),
    };
    match add_player(&req.state().pg_pool, &name).await {
        Ok(player) => player.build_response(),
        Err(_) => CustomError::new_player_already_existing_error(&name).into(),
    }
}
