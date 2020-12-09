-- Create players table
CREATE TABLE players (
  id SERIAL PRIMARY KEY,
  player_name VARCHAR NOT NULL,
  games_played INTEGER NOT NULL,
  games_won INTEGER NOT NULL
)
