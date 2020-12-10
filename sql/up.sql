-- Create players table
CREATE TABLE players (
  id SERIAL PRIMARY KEY,
  player_name VARCHAR(50) NOT NULL,
  games_played INTEGER NOT NULL,
  games_won INTEGER NOT NULL,
  UNIQUE(player_name)
);
