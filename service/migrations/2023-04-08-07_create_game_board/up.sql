CREATE TABLE IF NOT EXISTS game_board (
	game_id TEXT NOT NULL,
  tile_id INT NOT NULL,
  tile_index INT NOT NULL,
  PRIMARY KEY (game_id, tile_id)
);
