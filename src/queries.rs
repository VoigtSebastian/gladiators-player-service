use crate::player::{Player, PlayerName};
use async_std::stream::StreamExt;
use sqlx::postgres::PgPool;

/// Retrieves an user from the database by its id
/// (id in the Player struct and database).
///
/// Returns None if there is no such player in the database.
/// Returns the Player if it is in the database.
pub async fn query_player_by_id(connection: &PgPool, id: i32) -> Option<Player> {
    sqlx::query_as::<_, Player>("SELECT * FROM players WHERE id = $1;")
        .bind(id)
        .fetch_optional(connection)
        .await
        .unwrap_or(None)
}

/// Retrieves an user from the database by its name
/// (player_name in the Player struct and database).
///
/// Returns None if there is no such player in the database.
///
/// Returns the Player if it is in the database.
pub async fn query_player_by_name(connection: &PgPool, player_name: &PlayerName) -> Option<Player> {
    sqlx::query_as::<_, Player>("SELECT * FROM players WHERE player_name = $1;")
        .bind(&player_name.name())
        .fetch_optional(connection)
        .await
        .unwrap_or(None)
}

/// Returns **ALL** players that are stored in the database
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

/// Returns a range of players by their id.
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

/// Adds a new player to the database.
/// Returns an error if the specified Player is already in the database
/// (if the name of the Player is already in use).
pub async fn add_player(connection: &PgPool, player_name: &PlayerName) -> Result<Player, ()> {
    match query_player_by_name(connection, player_name).await {
        Some(_) => return Err(()),
        None => match sqlx::query("INSERT INTO players (id, player_name, games_played, games_won) VALUES (DEFAULT, $1, 0, 0);")
            .bind(&player_name.name())
            .execute(connection).await {
                Ok(_) => match query_player_by_name(connection, player_name).await {
                            Some(player) => Ok(player),
                            None => Err(()),
                },
                Err(_) => Err(()),
            },
    }
}

/// Updates games_played and games_won of the specified Player
///
/// The player is selected by its id field.
///
/// This function assumes that the player is part of the database and does not
/// do a lookup before execution.
async fn update_player_stats(
    connection: &PgPool,
    player: &Player,
) -> Result<sqlx::postgres::PgDone, sqlx::Error> {
    sqlx::query("UPDATE players SET (games_played, games_won) = ($1, $2) WHERE id = $3;")
        .bind(player.games_played)
        .bind(player.games_won)
        .bind(&player.id)
        .execute(connection)
        .await
}

/// Updates a Players games_played and games_won field, returns an Err if the
/// specified name is not in the database.
pub async fn player_played_round(
    connection: &PgPool,
    player_name: &PlayerName,
    won: bool,
) -> Result<Player, ()> {
    match query_player_by_name(connection, player_name).await {
        Some(mut player) => {
            player.games_played += 1;
            if won {
                player.games_won += 1;
            }
            match update_player_stats(connection, &player).await {
                Ok(_) => Ok(player),
                Err(_) => Err(()),
            }
        }
        None => Err(()),
    }
}
