CREATE TABLE IF NOT EXISTS game_hand (
	game_id TEXT NOT NULL,
  player_id TEXT NOT NULL,
  tile_id INT NOT NULL,
  set_id TEXT NULL,
  concealed INT NOT NULL,
  tile_index INT NOT NULL,
  PRIMARY KEY (game_id, tile_id)
);
