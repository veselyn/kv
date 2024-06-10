CREATE TABLE keys (
    id TEXT PRIMARY KEY,
    type TEXT NOT NULL CHECK (type IN ('json')),
    value TEXT NOT NULL
) STRICT
