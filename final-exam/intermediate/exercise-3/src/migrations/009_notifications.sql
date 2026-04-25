CREATE TABLE notifications (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    body TEXT NOT NULL DEFAULT ''
);
