mod player;
mod routes;
use routes::{echo_player, echo_players};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/echo/player").post(echo_player);
    app.at("/echo/players").post(echo_players);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
