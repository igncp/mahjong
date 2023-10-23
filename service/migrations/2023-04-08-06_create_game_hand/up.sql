CREATE TABLE IF NOT EXISTS game_hand (
    concealed INT NOT NULL,
    game_id TEXT NOT NULL REFERENCES game (id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES player (id) ON DELETE CASCADE,
    set_id TEXT NULL,
    tile_id INT NOT NULL,
    tile_index INT NOT NULL,
    PRIMARY KEY (game_id, tile_id)
);
