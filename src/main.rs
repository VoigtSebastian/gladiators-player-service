mod error;
mod player;
mod routes;
mod state;
use async_std::future::timeout;
use routes::*;
use sqlx::postgres::PgPoolOptions;
use state::State;
use std::time::Duration;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let connect = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:unsecure_password@localhost/gladiators_player_service");
    let pool = timeout(Duration::from_secs(5), connect).await??;

    let mut app = tide::with_state(State::new(pool));
    app.at("/echo/player").post(echo_player);
    app.at("/echo/players").post(echo_players);
    app.at("/player/:id").get(player_lookup_by_id);
    app.at("/player/name/:name").get(player_lookup_by_name);
    app.at("/player/won/:name").post(player_won);
    app.at("/player/played/:name").post(player_played);
    app.at("/players").get(players_lookup);
    app.with(driftwood::DevLogger);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
