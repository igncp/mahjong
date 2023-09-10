CREATE TABLE IF NOT EXISTS game (
    created_at TIMESTAMP NOT NULL,
    id TEXT PRIMARY KEY UNIQUE NOT NULL,
    name TEXT NOT NULL,
    phase TEXT NOT NULL,
    round_claimed_by TEXT NULL,
    round_claimed_from TEXT NULL,
    round_claimed_id INT NULL,
    round_dealer_index INT NOT NULL,
    round_index INT NOT NULL,
    round_player_index INT NOT NULL,
    round_wall_tile_drawn INT NULL,
    round_wind TEXT NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version TEXT NOT NULL
);
