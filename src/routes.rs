use crate::error::CustomError;
use crate::player::{Player, PlayerName};
use crate::queries::*;
use crate::state::State;
use actix_web::web::{Data, Json, Path};
use actix_web::{Either, Responder, Result};
use tracing::{info, warn};

// curl localhost:5050/echo/player \
// -d '{ "id": 1, "player_name": "test", "games_played": 2, "games_won": 3}'
pub async fn echo_player(Json(player): Json<Player>) -> impl Responder {
    player
}

// curl localhost:5050/echo/players \
// -d '[{ "id": 1, "player_name": "test", "games_played": 2, "games_won": 3}, { "id": 2, "name": "test", "games_played": 2, "games_won": 3}]'
pub async fn echo_players(players: Json<Vec<Player>>) -> Result<Json<Vec<Player>>> {
    Ok(players)
}

// curl localhost:5050/player/id/:id
pub async fn player_lookup_by_id(
    data: Data<State>,
    Path(id): Path<i32>,
) -> Either<impl Responder, impl Responder> {
    match query_player_by_id(&data.pg_pool, id).await {
        Some(player) => Either::A(player),
        None => {
            info!("Could not find a player with id ({}) in database", id);
            Either::B(CustomError::new_player_not_found_by_id_error(id))
        }
    }
}

// curl localhost:5050/player/name/:name
pub async fn player_lookup_by_name(
    data: Data<State>,
    Path(name): Path<String>,
) -> Either<impl Responder, impl Responder> {
    let player_name = match PlayerName::new(&name) {
        Ok(player_name) => player_name,
        Err(_) => {
            info!("Could not find a player with name ({}) in database", name);
            return Either::B(CustomError::player_name_has_wrong_format(&name));
        }
    };

    match query_player_by_name(&data.pg_pool, &player_name).await {
        Some(player) => Either::A(player),
        None => {
            info!("Could not find a player with name ({}) in database", name);
            Either::B(CustomError::new_player_not_found_by_name_error(name))
        }
    }
}

// curl localhost:5050/players
pub async fn players_lookup(data: Data<State>) -> Result<Json<Vec<Player>>> {
    Ok(Json(query_players(&data.pg_pool).await))
}

pub async fn player_played(
    data: Data<State>,
    Path(name): Path<String>,
) -> Either<impl Responder, impl Responder> {
    let player_name = match PlayerName::new(&name) {
        Ok(player_name) => player_name,
        Err(_) => {
            info!("Could not find a player with name ({}) in database", name);
            return Either::B(CustomError::player_name_has_wrong_format(&name));
        }
    };

    match player_played_round(&data.pg_pool, &player_name, false).await {
        Ok(player) => Either::A(player),
        Err(_) => {
            warn!("Could not find a player with name ({}) in database", name);
            return Either::B(CustomError::player_name_has_wrong_format(&name));
        }
    }
}

pub async fn player_won(
    data: Data<State>,
    Path(name): Path<String>,
) -> Either<impl Responder, impl Responder> {
    let player_name = match PlayerName::new(&name) {
        Ok(player_name) => player_name,
        Err(_) => {
            info!("Could not find a player with name ({}) in database", name);
            return Either::B(CustomError::player_name_has_wrong_format(&name));
        }
    };

    match player_played_round(&data.pg_pool, &player_name, true).await {
        Ok(player) => Either::A(player),
        Err(_) => {
            info!("Could not find a player with name ({}) in database", name);
            Either::B(CustomError::new_player_not_found_by_name_error(name))
        }
    }
}

pub async fn register_player(
    data: Data<State>,
    Path(name): Path<String>,
) -> Either<impl Responder, impl Responder> {
    let player_name = match PlayerName::new(&name) {
        Ok(player_name) => player_name,
        Err(_) => {
            info!("Could not find a player with name ({}) in database", name);
            return Either::B(CustomError::player_name_has_wrong_format(&name));
        }
    };

    match add_player(&data.pg_pool, &player_name).await {
        Ok(player) => Either::A(player),
        Err(_) => {
            info!("Player ({}) is already existing in database", name);
            Either::B(CustomError::new_player_already_existing_error(&name))
        }
    }
}
