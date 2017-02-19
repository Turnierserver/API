CREATE TABLE users (
  id SERIAL PRIMARY KEY NOT NULL,
  username VARCHAR NOT NULL UNIQUE,
  email VARCHAR NOT NULL,
  firstname VARCHAR,
  lastname VARCHAR,
  pwhash TEXT,
  admin BOOLEAN NOT NULL DEFAULT 'f',
  name_public BOOLEAN NOT NULL DEFAULT 't'
);

CREATE TABLE ais (
  id SERIAL PRIMARY KEY NOT NULL,
  user_id INTEGER REFERENCES users(id) NOT NULL,
  name VARCHAR NOT NULL,
  description VARCHAR,
  elo DOUBLE PRECISION NOT NULL DEFAULT 1200
);

WITH admin AS (
  INSERT INTO users (username, email, admin, pwhash)
    VALUES ('admin', 'admin@ad.min', TRUE, '$2y$12$pipJrd3RKt8uCzmIDlxaVe//zq9lNuKFyT9QggV9J0jGwZI6ipiIO')
    RETURNING *
)
INSERT INTO ais (name, user_id)
  SELECT 'admin ai', id from admin;

INSERT INTO users (username, email, admin)
  VALUES ('user', 'user@example.com', FALSE);