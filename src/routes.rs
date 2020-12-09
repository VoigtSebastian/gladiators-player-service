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
