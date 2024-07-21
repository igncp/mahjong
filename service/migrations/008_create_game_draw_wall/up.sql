CREATE TABLE IF NOT EXISTS game_draw_wall (
    game_id TEXT NOT NULL REFERENCES game (id) ON DELETE CASCADE,
    tile_id INT NOT NULL,
    tile_index INT NOT NULL,
    place TEXT NOT NULL,
    PRIMARY KEY (game_id, tile_id, place)
);
