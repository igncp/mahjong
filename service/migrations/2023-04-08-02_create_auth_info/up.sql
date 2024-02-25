CREATE TABLE IF NOT EXISTS auth_info_providers (
    id TEXT PRIMARY KEY
);

INSERT INTO auth_info_providers (id) VALUES ('email');
INSERT INTO auth_info_providers (id) VALUES ('github');
INSERT INTO auth_info_providers (id) VALUES ('anonymous');

CREATE TABLE IF NOT EXISTS auth_info (
    provider TEXT NOT NULL REFERENCES auth_info_providers (
        id
    ) ON DELETE CASCADE,
    role TEXT NOT NULL,
    user_id TEXT NOT NULL UNIQUE PRIMARY KEY REFERENCES player (
        id
    ) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS auth_info_email (
    hashed_pass TEXT NOT NULL,
    user_id TEXT NOT NULL UNIQUE PRIMARY KEY REFERENCES auth_info (
        user_id
    ) ON DELETE CASCADE,
    username TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS auth_info_github (
    token TEXT NULL,
    user_id TEXT NOT NULL UNIQUE PRIMARY KEY REFERENCES auth_info (
        user_id
    ) ON DELETE CASCADE,
    username TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS auth_info_anonymous (
    hashed_token TEXT UNIQUE NOT NULL,
    user_id TEXT NOT NULL UNIQUE PRIMARY KEY REFERENCES auth_info (
        user_id
    ) ON DELETE CASCADE
);
