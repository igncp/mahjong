CREATE TABLE IF NOT EXISTS auth_info_providers (
    id TEXT PRIMARY KEY
);

INSERT INTO auth_info_providers (id) VALUES ('email');
INSERT INTO auth_info_providers (id) VALUES ('github');

CREATE TABLE IF NOT EXISTS auth_info (
    provider TEXT NOT NULL REFERENCES auth_info_providers (
        id
    ) ON DELETE CASCADE,
    role TEXT NOT NULL,
    user_id TEXT NOT NULL UNIQUE PRIMARY KEY
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
