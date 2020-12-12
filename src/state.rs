use sqlx::Pool;
use sqlx::Postgres;

#[derive(Debug, Clone)]
pub struct State {
    pub pg_pool: Pool<Postgres>,
}

impl State {
    pub fn new(pg_pool: Pool<Postgres>) -> State {
        State { pg_pool: pg_pool }
    }
}
