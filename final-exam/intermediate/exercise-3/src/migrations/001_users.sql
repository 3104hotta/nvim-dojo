CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL DEFAULT '',
    password_hash TEXT NOT NULL DEFAULT ''
);
