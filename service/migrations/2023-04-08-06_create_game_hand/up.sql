CREATE TABLE IF NOT EXISTS game_hand (
    concealed INT NOT NULL,
    game_id TEXT NOT NULL,
    player_id TEXT NOT NULL,
    set_id TEXT NULL,
    tile_id INT NOT NULL,
    tile_index INT NOT NULL,
    PRIMARY KEY (game_id, tile_id)
);
