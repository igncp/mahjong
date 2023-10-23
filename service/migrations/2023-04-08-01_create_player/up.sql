CREATE TABLE IF NOT EXISTS player (
    created_at TIMESTAMP NOT NULL,
    -- Every authenticated user is associated with a player, but some players
    -- don't have login info (e.g. AI players)
    id TEXT PRIMARY KEY UNIQUE NOT NULL,
    is_ai INT NOT NULL,
    name TEXT NOT NULL
);
