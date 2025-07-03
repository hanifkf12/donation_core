-- Add migration script here

CREATE TABLE IF NOT EXISTS users (
    id uuid primary key not null,
    username varchar(100) not null unique,
    email varchar(100) not null unique,
    password text not null,
    is_verified boolean default false,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    deleted_at timestamp null default null
    );

CREATE INDEX IF NOT EXISTS users_username_idx ON users (username);
CREATE INDEX IF NOT EXISTS users_email_idx ON users (email);