# gladiators-player-service
The Player Service of Gladiators

Saves unique player-names and their id in a postgres database.

## Technology
- Written in Rust
  - tide as HTML-Server
  - async-std as asynchronous runtime
  - sqlx to interface with postgres
- Postgres database
  - Running as a docker-container

## Postgres
### Docker
To start postgres as a docker container run the command below.
```sh
docker run \
    --name=player_db \
    -e POSTGRES_DB=gladiators_player_service \
    -e POSTGRES_PASSWORD=unsecure_password \
    -p 5432:5432 \
    -d postgres:13-alpine
```

**Obvious heads-up** there is nothing secure about the way the password is
hard-coded into the docker-command and the script.
I will change this behavior in a future commit, even if, in the end, this is
just an educational project.

### Initialization
To create the 'players' table and populate the database with test-data, run.
```sh
psql -h localhost -p 5432 -U postgres -d gladiators_player_service -a -f sql/up.sql
psql -h localhost -p 5432 -U postgres -d gladiators_player_service -a -f sql/example_data.sql
```
