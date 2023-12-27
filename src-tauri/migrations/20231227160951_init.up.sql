-- Your SQL goes here


-- Table for Pokemon Version
CREATE TABLE IF NOT EXISTS Version (
    name TEXT PRIMARY KEY NOT NULL,
    generation INTEGER NOT NULL
);

-- Table for pokemon regions
CREATE TABLE IF NOT EXISTS Region (
    name TEXT PRIMARY KEY NOT NULL
);

-- Table for pokeballs
CREATE TABLE IF NOT EXISTS Ball (
    name TEXT PRIMARY KEY NOT NULL
);

-- Table for types
CREATE TABLE IF NOT EXISTS Type (
    name TEXT PRIMARY KEY NOT NULL,
    color TEXT NOT NULL
);

-- Table for Battle Types
CREATE TABLE IF NOT EXISTS Battle_Type (
    name TEXT PRIMARY KEY NOT NULL
);

-- Table for Catch Types
CREATE TABLE IF NOT EXISTS Catch_Type (
    name TEXT PRIMARY KEY NOT NULL
);

-- Table for Items
CREATE TABLE IF NOT EXISTS Item (
    name TEXT PRIMARY KEY NOT NULL
);

-- Table for pokemon locations
CREATE TABLE IF NOT EXISTS Location (
    name TEXT NOT NULL,
    region TEXT NOT NULL,
    PRIMARY KEY (name, region),
    FOREIGN KEY (region) REFERENCES Region(name) ON DELETE RESTRICT
);

-- Table for Pokemon Species
CREATE TABLE IF NOT EXISTS Species (
    name TEXT NOT NULL,
    dex_no INTEGER NOT NULL,
    generation INTEGER NOT NULL,
    type1 TEXT NOT NULL,
    type2 TEXT,
    PRIMARY KEY (name),
    FOREIGN KEY (type1) REFERENCES Type(name) ON DELETE RESTRICT,
    FOREIGN KEY (type2) REFERENCES Type(name) ON DELETE RESTRICT
);

-- Table for individual playthroughs
CREATE TABLE IF NOT EXISTS Playthrough (
    id_no TEXT NOT NULL,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    adventure_started TEXT NOT NULL,
    PRIMARY KEY (id_no)
    -- FOREIGN KEY (version) REFERENCES Version(name) ON DELETE RESTRICT
);

-- Table for Team Member
CREATE TABLE IF NOT EXISTS Team_Member (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    playthrough_id_no TEXT NOT NULL,
    slot INTEGER NOT NULL,
    nickname TEXT,
    caught_date TEXT NOT NULL,
    caught_location_name TEXT NOT NULL,
    caught_location_region TEXT NOT NULL,
    caught_species_name TEXT NOT NULL,
    caught_level INTEGER NOT NULL,
    ball TEXT NOT NULL,
    gender TEXT NOT NULL,
    UNIQUE (playthrough_id_no, slot),
    FOREIGN KEY (playthrough_id_no) REFERENCES Playthrough(id_no) ON DELETE RESTRICT,
    FOREIGN KEY (caught_location_name, caught_location_region) REFERENCES Location(name, region) ON DELETE RESTRICT,
    FOREIGN KEY (ball) REFERENCES Ball(name) ON DELETE RESTRICT,
    FOREIGN KEY (caught_species_name) REFERENCES Species(name) ON DELETE RESTRICT,
    CHECK (slot >= 1 AND slot <= 6),
    CHECK (gender IN ("M","F","N")),
    CHECK (caught_level >= 1 AND caught_level <= 100)
);

-- Trainer Class Table
CREATE TABLE IF NOT EXISTS Trainer_Class (
    name TEXT PRIMARY KEY NOT NULL
);

-- Trainer Table
CREATE TABLE IF NOT EXISTS Trainer (
    name TEXT NOT NULL,
    class TEXT NOT NULL,
    PRIMARY KEY (name, class),
    FOREIGN KEY (class) REFERENCES Trainer_Class(name) ON DELETE RESTRICT
);

-- event table
CREATE TABLE IF NOT EXISTS Event (
    no INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    playthrough_id_no TEXT NOT NULL,
    location_name TEXT NOT NULL,
    location_region TEXT NOT NULL,
    FOREIGN KEY (playthrough_id_no) REFERENCES Playthrough(id_no) ON DELETE RESTRICT,
    FOREIGN KEY (location_name, location_region) REFERENCES Location(name, region) ON DELETE RESTRICT
);

-- battle event table
CREATE TABLE IF NOT EXISTS Battle_Event (
    no INTEGER NOT NULL PRIMARY KEY,
    battle_type TEXT NOT NULL,
    opponent1_name TEXT NOT NULL,
    opponent1_class TEXT NOT NULL,
    opponent2_name TEXT,
    opponent2_class TEXT,
    partner_name TEXT,
    partner_class TEXT,
    round INTEGER NOT NULL,
    lost BOOLEAN DEFAULT 0 NOT NULL,
    FOREIGN KEY (no) REFERENCES Event(no) ON DELETE RESTRICT,
    FOREIGN KEY (battle_type) REFERENCES Battle_Type(name) ON DELETE RESTRICT,
    FOREIGN KEY (opponent1_name, opponent1_class) REFERENCES Trainer(name, class) ON DELETE RESTRICT,
    FOREIGN KEY (opponent2_name, opponent2_class) REFERENCES Trainer(name, class) ON DELETE RESTRICT,
    FOREIGN KEY (partner_name, partner_class) REFERENCES Trainer(name, class) ON DELETE RESTRICT,
    CHECK (lost IN (0,1))
);

-- events caused by items
CREATE TABLE IF NOT EXISTS Item_Event (
    no INTEGER NOT NULL PRIMARY KEY,
    item TEXT NOT NULL,
    FOREIGN KEY (no) REFERENCES Event(no) ON DELETE RESTRICT,
    FOREIGN KEY (item) REFERENCES Item(name) ON DELETE RESTRICT
);

-- catch event
CREATE TABLE IF NOT EXISTS Catch_Event (
    no INTEGER NOT NULL PRIMARY KEY,
    catch_type TEXT NOT NULL,
    FOREIGN KEY (no) REFERENCES Event(no) ON DELETE RESTRICT,
    FOREIGN KEY (catch_type) REFERENCES Catch_Type(name) ON DELETE RESTRICT
);

-- Team Member Changes
CREATE TABLE IF NOT EXISTS Team_Member_Change (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    team_member_id INTEGER NOT NULL,
    event_no INTEGER NOT NULL,
    level INTEGER,
    species_name TEXT,
    FOREIGN KEY (team_member_id) REFERENCES Team_Member(id) ON DELETE RESTRICT,
    FOREIGN KEY (event_no) REFERENCES Event(no) ON DELETE RESTRICT,
    CHECK (level >= 1 AND level <= 100),
    FOREIGN KEY (species_name) REFERENCES Species(name) ON DELETE RESTRICT
);