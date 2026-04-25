CREATE TABLE sessions (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    token TEXT NOT NULL DEFAULT ''
);
