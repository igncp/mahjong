CREATE TABLE IF NOT EXISTS game_score (
	game_id TEXT NOT NULL,
  player_id TEXT NOT NULL,
  score INT NOT NULL DEFAULT 0,
  PRIMARY KEY (game_id, player_id)
);
