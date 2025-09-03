-- Add up migration script here
create table rooms
(
    id uuid primary key default gen_random_uuid(),
    name varchar(240) not null,
    password_hash text not null,
    created_at timestamptz not null default now()
);
