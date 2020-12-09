use crate::gladiator::{gladiator_vector_response, Gladiator};
use tide::Request;

// curl localhost:8080/echo/gladiator \
// -d '{ "id": 1, "player_name": "test", "games_played": 2, "games_won": 3}'
pub async fn echo_gladiator(mut req: Request<()>) -> tide::Result {
    let gladiator: Gladiator = req.body_json().await?;
    gladiator.build_response()
}

// curl localhost:8080/echo/gladiators \
// -d '[{ "id": 1, "player_name": "test", "games_played": 2, "games_won": 3}, { "id": 2, "name": "test", "games_played": 2, "games_won": 3}]'
pub async fn echo_gladiators(mut req: Request<()>) -> tide::Result {
    let gladiators: Vec<Gladiator> = req.body_json().await?;
    gladiator_vector_response(&gladiators)
}
