use tide::convert::{Deserialize, Serialize};
use tide::http::mime;
use tide::{Body, Request, Response, StatusCode};

#[derive(Debug, Deserialize, Serialize)]
struct Gladiator {
    id: u32,
    player_name: String,
    games_played: u32,
    games_won: u32,
}

impl Gladiator {
    fn build_response(&self) -> tide::Result {
        let mut response = Response::new(StatusCode::Ok);
        response.set_content_type(mime::JSON);
        response.set_body(Body::from_json(&self)?);
        Ok(response)
    }
}

fn gladiator_vector_response(gladiators: &Vec<Gladiator>) -> tide::Result {
    let mut response = Response::new(StatusCode::Ok);
    response.set_content_type(mime::JSON);
    response.set_body(Body::from_json(&gladiators)?);
    Ok(response)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/echo/gladiator").post(echo_gladiator);
    app.at("/echo/gladiators").post(echo_gladiators);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

// curl localhost:8080/echo/gladiator \
// -d '{ "id": 1, "player_name": "test", "games_played": 2, "games_won": 3}'
async fn echo_gladiator(mut req: Request<()>) -> tide::Result {
    let gladiator: Gladiator = req.body_json().await?;
    gladiator.build_response()
}

// curl localhost:8080/echo/gladiators \
// -d '[{ "id": 1, "player_name": "test", "games_played": 2, "games_won": 3}, { "id": 2, "name": "test", "games_played": 2, "games_won": 3}]'
async fn echo_gladiators(mut req: Request<()>) -> tide::Result {
    let gladiators: Vec<Gladiator> = req.body_json().await?;
    gladiator_vector_response(&gladiators)
}
