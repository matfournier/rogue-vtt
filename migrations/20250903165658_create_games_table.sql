-- Add up migration script here
create table games
(
    id uuid primary key default gen_random_uuid(),
    name varchar(240) not null,
    room_id uuid not null references rooms(id) on delete cascade,
    created_at timestamptz not null default now()
);