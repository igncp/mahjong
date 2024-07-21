CREATE TABLE IF NOT EXISTS game_score (
    game_id TEXT NOT NULL REFERENCES game (id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES player (id) ON DELETE CASCADE,
    score INT NOT NULL DEFAULT 0,
    PRIMARY KEY (game_id, player_id)
);
