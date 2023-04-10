CREATE TABLE IF NOT EXISTS game_player (
	game_id TEXT NOT NULL,
  player_id TEXT NOT NULL,
  player_index INT NOT NULL,
  PRIMARY KEY (game_id, player_id)
);
