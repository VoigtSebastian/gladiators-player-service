mod error;
mod player;
mod routes;
mod state;
use routes::{echo_player, echo_players, player_lookup, players_lookup};
use sqlx::postgres::PgPoolOptions;
use state::State;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:unsecure_password@localhost/gladiators_player_service")
        .await?;

    let mut app = tide::with_state(State { pg_pool: pool });
    app.at("/echo/player").post(echo_player);
    app.at("/echo/players").post(echo_players);
    app.at("/player/:id").get(player_lookup);
    app.at("/players").get(players_lookup);
    app.with(driftwood::DevLogger);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
