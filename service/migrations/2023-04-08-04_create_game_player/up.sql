CREATE TABLE IF NOT EXISTS game_player (
	game_id TEXT NOT NULL REFERENCES game(id),
  player_id TEXT NOT NULL REFERENCES player(id),
  player_index INT NOT NULL,
  PRIMARY KEY (game_id, player_id)
);
