CREATE TABLE follows (
    id INTEGER PRIMARY KEY,
    follower_id INTEGER NOT NULL,
    followee_id INTEGER NOT NULL
);
