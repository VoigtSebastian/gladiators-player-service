mod gladiator;
mod routes;
use routes::{echo_gladiator, echo_gladiators};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/echo/gladiator").post(echo_gladiator);
    app.at("/echo/gladiators").post(echo_gladiators);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
