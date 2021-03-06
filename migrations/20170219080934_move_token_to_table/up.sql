ALTER TABLE users DROP COLUMN token;

CREATE TABLE tokens (
    id UUID PRIMARY KEY NOT NULL,
    user_id INTEGER REFERENCES users(id) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL
);