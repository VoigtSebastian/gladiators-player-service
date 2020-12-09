use sqlx::Pool;
use sqlx::Postgres;

#[derive(Debug, Clone)]
pub struct State {
    pub pg_pool: Pool<Postgres>,
}
