use async_std::stream::StreamExt;
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

pub async fn query_player_by_id(connection: &PgPool, id: i32) -> Option<Player> {
    sqlx::query_as::<_, Player>("SELECT * FROM players WHERE id = $1;")
        .bind(id)
        .fetch_optional(connection)
        .await
        .unwrap_or(None)
}

pub async fn query_player_by_name(connection: &PgPool, name: &String) -> Option<Player> {
    sqlx::query_as::<_, Player>("SELECT * FROM players WHERE player_name = $1;")
        .bind(name)
        .fetch_optional(connection)
        .await
        .unwrap_or(None)
}

pub async fn query_players(connection: &PgPool) -> Vec<Player> {
    let mut players: Vec<Player> = vec![];
    let mut query = sqlx::query_as::<_, Player>("SELECT * FROM players;").fetch(connection);

    while let Some(player) = query.next().await {
        match player {
            Ok(player) => players.push(player),
            Err(_) => (),
        }
    }
    players
}

pub async fn query_players_range(connection: &PgPool, from: i32, to: i32) -> Vec<Player> {
    let mut players: Vec<Player> = vec![];
    let mut query = sqlx::query_as::<_, Player>("SELECT * FROM players WHERE id > $1 AND id < $2;")
        .bind(from)
        .bind(to)
        .fetch(connection);

    while let Some(player) = query.next().await {
        match player {
            Ok(player) => players.push(player),
            Err(_) => (),
        }
    }
    players
}

pub async fn add_player(connection: &PgPool, name: String) -> Result<Player, ()> {
    match query_player_by_name(connection, &name).await {
        Some(_) => return Err(()),
        None => match sqlx::query("INSERT INTO players (id, player_name, games_played, games_won) VALUES (DEFAULT, '$1', 0, 0);")
            .bind(&name)
            .execute(connection).await {
                Ok(_) => match query_player_by_name(connection, &name).await {
                            Some(player) => Ok(player),
                            None => Err(())
                },
                Err(_) => Err(()),
            },
    }
}

pub async fn update_player(connection: &PgPool, name: String, won: bool) -> Result<Player, ()> {
    match query_player_by_name(connection, &name).await {
        Some(mut player) => {
            player.games_played += 1;
            if won {
                player.games_won += 1;
            }
            match sqlx::query(
                "UPDATE players SET (games_played, games_won) = ($1, $2) WHERE player_name = $3;"
            )
            .bind(player.games_played)
            .bind(player.games_won)
            .bind(&name)
            .execute(connection)
            .await
            {
                Ok(_) => Ok(player),
                Err(_) => Err(()),
            }
        }
        None => Err(()),
    }
}

pub fn player_vector_response(player: &Vec<Player>) -> tide::Result {
    let mut response = Response::new(StatusCode::Ok);
    response.set_content_type(mime::JSON);
    response.set_body(Body::from_json(&player)?);
    Ok(response)
}
