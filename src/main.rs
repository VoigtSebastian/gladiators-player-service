mod error;
mod player;
mod queries;
mod routes;
mod state;
use actix_web::web::{get, post, scope};
use actix_web::{App, HttpServer};
use routes::*;
use sqlx::postgres::PgPoolOptions;
use state::State;
use std::option_env;
use tracing::{info, instrument, Level};
use tracing_actix_web::TracingLogger;

static DATABASE_UP: &'static str = include_str!("../sql/up.sql");

#[instrument]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(option_env!("DATABASE_URL").unwrap_or(
            "postgresql://postgres:unsecure_password@localhost/gladiators_player_service",
        ))
        .await
        .expect("Could not open database-connection");
    info!("Successfully connected to postgres database");

    sqlx::query(DATABASE_UP)
        .execute(&pool)
        .await
        .expect("Could not initialize player-table");
    info!("Successfully initialized player table");

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .data(State::new(pool.clone()))
            .service(
                scope("/player")
                    .route("/{id}", get().to(player_lookup_by_id))
                    .route("/name/{name}", get().to(player_lookup_by_name))
                    .route("/register/{name}", post().to(register_player))
                    .route("/won/{name}", post().to(player_won))
                    .route("/played/{name}", post().to(player_played)),
            )
            .route("/players", get().to(players_lookup))
    })
    .bind("0.0.0.0:5050")?
    .run()
    .await
}
