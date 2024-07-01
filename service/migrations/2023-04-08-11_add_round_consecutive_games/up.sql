ALTER TABLE game ADD COLUMN round_consecutive_same_seats INT NOT NULL DEFAULT 0;
ALTER TABLE game ADD COLUMN round_east_player_index INT NOT NULL DEFAULT 0;
