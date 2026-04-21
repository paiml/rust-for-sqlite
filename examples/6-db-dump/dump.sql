PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL, email TEXT, created_at TEXT);
INSERT INTO users VALUES(1,'Alice','alice@example.com',NULL);
INSERT INTO users VALUES(2,'Bob','bob@example.com',NULL);
COMMIT;
