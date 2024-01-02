-- Your SQL goes here
PRAGMA foreign_keys = ON;

-- catch event
ALTER TABLE Catch_Event RENAME TO old_Catch_Event;
CREATE TABLE Catch_Event (
    no INTEGER NOT NULL PRIMARY KEY,
    catch_type TEXT NOT NULL,
    team_member_id INTEGER NOT NULL,
    FOREIGN KEY (no) REFERENCES Event(no) ON DELETE RESTRICT,
    FOREIGN KEY (catch_type) REFERENCES Catch_Type(name) ON DELETE RESTRICT,
    FOREIGN KEY (team_member_id) REFERENCES Team_Member(id) ON DELETE RESTRICT
);
INSERT INTO Catch_Event (no, catch_type, team_member_id)
    SELECT no, catch_type, ROW_NUMBER() OVER(ORDER BY no)
    FROM old_Catch_Event;
DROP TABLE old_Catch_Event;