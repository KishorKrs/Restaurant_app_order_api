CREATE TABLE IF NOT EXISTS orders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_number INTEGER NOT NULL,
    item TEXT NOT NULL,
    cook_time INTEGER NOT NULL
);