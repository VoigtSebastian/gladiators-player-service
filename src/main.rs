mod error;
mod player;
mod queries;
mod routes;
mod state;
use routes::*;
use sqlx::postgres::PgPoolOptions;
use state::State;
use std::option_env;

static DATABASE_UP: &'static str = include_str!("../sql/up.sql");

#[async_std::main]
async fn main() -> tide::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(option_env!("DATABASE_URL")
            .unwrap_or("postgresql://postgres:unsecure_password@localhost/gladiators_player_service"))
        .await?;

    sqlx::query(DATABASE_UP)
        .execute(&pool)
        .await?;

    let mut app = tide::with_state(State::new(pool));
    app.at("/echo/player").post(echo_player);
    app.at("/echo/players").post(echo_players);
    app.at("/player/:id").get(player_lookup_by_id);
    app.at("/player/register/:name").post(register_player);
    app.at("/player/name/:name").get(player_lookup_by_name);
    app.at("/player/won/:name").post(player_won);
    app.at("/player/played/:name").post(player_played);
    app.at("/players").get(players_lookup);
    app.with(driftwood::DevLogger);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
