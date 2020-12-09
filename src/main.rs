mod player;
mod routes;
mod state;
use player::query_player;
use routes::{echo_player, echo_players};
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
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
