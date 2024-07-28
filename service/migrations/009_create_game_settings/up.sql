CREATE TABLE IF NOT EXISTS game_settings (
    ai_enabled BOOLEAN NOT NULL,
    auto_sort_players TEXT NOT NULL,
    auto_stop_claim_meld TEXT NOT NULL,
    discard_wait_ms INT NULL,
    fixed_settings BOOLEAN NOT NULL,
    game_id TEXT PRIMARY KEY UNIQUE NOT NULL REFERENCES game (
        id
    ) ON DELETE CASCADE,
    last_discard_time BIGINT NOT NULL,
    dead_wall BOOLEAN NOT NULL
);
