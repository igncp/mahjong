CREATE TABLE IF NOT EXISTS player (
	id TEXT PRIMARY KEY UNIQUE NOT NULL,
	is_ai INT NOT NULL,
	name TEXT NOT NULL
);
