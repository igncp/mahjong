CREATE TABLE IF NOT EXISTS game_player (
    game_id TEXT NOT NULL REFERENCES game (id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES player (id) ON DELETE CASCADE,
    player_index INT NOT NULL,
    PRIMARY KEY (game_id, player_id)
);
