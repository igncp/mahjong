CREATE TABLE IF NOT EXISTS auth_info (
	hashed_pass TEXT NOT NULL,
	role TEXT NOT NULL,
	user_id TEXT NOT NULL UNIQUE,
	username TEXT PRIMARY KEY UNIQUE NOT NULL
);
