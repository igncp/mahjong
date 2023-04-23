CREATE TABLE IF NOT EXISTS game_settings (
  ai_enabled INT NOT NULL,
  discard_wait_ms INT NULL,
  fixed_settings INT NOT NULL,
  game_id TEXT PRIMARY KEY UNIQUE NOT NULL,
  last_discard_time BIGINT NOT NULL
);
