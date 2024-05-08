-- Add migration script here

CREATE TABLE games(
    game_id uuid NOT NULL,
    PRIMARY KEY (game_id),
    current_level uuid,
    key_hash TEXT NOT NULL
)
