CREATE TABLE audit_log (
    id INTEGER PRIMARY KEY,
    actor_id INTEGER NOT NULL,
    action TEXT NOT NULL DEFAULT ''
);
