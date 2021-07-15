-- Your SQL goes here
-- create_boards up.sql
CREATE TABLE IF NOT EXISTS boards (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'utc')
);

-- seed db with some test data for local dev
INSERT INTO boards
(name)
VALUES
('Test board 1'),
('Test board 2'),
('Test board 3');
