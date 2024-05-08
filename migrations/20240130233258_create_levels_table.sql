-- Add migration script here
CREATE TABLE levels(
    level_id uuid NOT NULL,
    game_id uuid NOT NULL references games(game_id),
    description varchar(64) NOT NULL, 
    level_type smallint NOT NULL, 
    data jsonb NOT NULL,
    PRIMARY KEY(level_id)
)


