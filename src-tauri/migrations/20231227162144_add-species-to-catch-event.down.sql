-- Add down migration script here

-- This file should undo anything in `up.sql`
PRAGMA foreign_keys = ON;

-- catch event
ALTER TABLE Catch_Event RENAME TO old_Catch_Event;
CREATE TABLE Catch_Event (
    no INTEGER NOT NULL PRIMARY KEY,
    catch_type TEXT NOT NULL,
    FOREIGN KEY (no) REFERENCES Event(no) ON DELETE RESTRICT,
    FOREIGN KEY (catch_type) REFERENCES Catch_Type(name) ON DELETE RESTRICT
);
INSERT INTO Catch_Event (no, catch_type)
    SELECT no, catch_type
    FROM old_Catch_Event;
DROP TABLE old_Catch_Event;