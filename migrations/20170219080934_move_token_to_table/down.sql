DROP TABLE tokens;
ALTER TABLE users ADD COLUMN token UUID UNIQUE;