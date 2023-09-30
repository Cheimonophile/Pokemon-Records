-- Your SQL goes here



-- Table for Pokemon Version
CREATE TABLE Version (
    name TEXT PRIMARY KEY NOT NULL,
    generation INTEGER NOT NULL
);

-- Table for pokemon regions
CREATE TABLE Region (
    name TEXT PRIMARY KEY NOT NULL
);

-- Table for pokeballs
CREATE TABLE Ball (
    name TEXT PRIMARY KEY NOT NULL
);

-- Table for types
CREATE TABLE Type (
    name TEXT PRIMARY KEY NOT NULL,
    color TEXT NOT NULL
);

-- Table for pokemon locations
CREATE TABLE Location (
    name TEXT NOT NULL,
    region TEXT NOT NULL,
    PRIMARY KEY (name, region),
    FOREIGN KEY (region) REFERENCES Region(name) ON DELETE RESTRICT
);

-- Table for Pokemon Species
CREATE TABLE Species (
    dex_no INTEGER NOT NULL,
    name TEXT NOT NULL,
    form TEXT NOT NULL,
    generation INTEGER NOT NULL,
    PRIMARY KEY (dex_no, form)
);

-- Table for pokemon species game info
CREATE TABLE SpeciesVersion (
    species_dex_no INTEGER NOT NULL,
    species_form TEXT NOT NULL,
    version TEXT NOT NULL,
    type1 TEXT NOT NULL,
    type2 TEXT,
    color TEXT NOT NULL,
    PRIMARY KEY (species_dex_no, species_form, version),
    FOREIGN KEY (species_dex_no, species_form) REFERENCES Species(dex_no, form) ON DELETE RESTRICT,
    FOREIGN KEY (version) REFERENCES Version(name) ON DELETE RESTRICT,
    FOREIGN KEY (type1) REFERENCES Type(name) ON DELETE RESTRICT,
    FOREIGN KEY (type2) REFERENCES Type(name) ON DELETE RESTRICT
);

-- Table for individual playthroughs
CREATE TABLE Playthrough (
    id_no TEXT NOT NULL,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    adventure_started TEXT NOT NULL,
    PRIMARY KEY (id_no),
    FOREIGN KEY (version) REFERENCES Version(name) ON DELETE RESTRICT
);

-- Table for Team Member
CREATE TABLE Team_Member (
    playthrough_id_no TEXT NOT NULL,
    slot TEXT NOT NULL,
    nickname TEXT,
    caught_date TEXT NOT NULL,
    caught_location_name TEXT NOT NULL,
    caught_location_region TEXT NOT NULL,
    caught_level INTEGER NOT NULL,
    ball TEXT NOT NULL,
    gender TEXT NOT NULL,
    PRIMARY KEY (playthrough_id_no, slot),
    FOREIGN KEY (playthrough_id_no) REFERENCES Playthrough(id_no) ON DELETE RESTRICT,
    FOREIGN KEY (caught_location_name, caught_location_region) REFERENCES Location(name, region) ON DELETE RESTRICT,
    FOREIGN KEY (ball) REFERENCES Ball(name) ON DELETE RESTRICT,
    CHECK (slot >= 1 AND slot <= 6),
    CHECK (gender IN ("M","F","N")),
    CHECK (caught_level >= 1 AND caught_level <= 100)
);

-- Trainer Class Table
CREATE TABLE Trainer_Class (
    name TEXT PRIMARY KEY NOT NULL
);

-- Trainer Table
CREATE TABLE Trainer (
    name TEXT NOT NULL,
    class TEXT NOT NULL,
    PRIMARY KEY (name, class),
    FOREIGN KEY (class) REFERENCES Trainer_Class(name) ON DELETE RESTRICT
);

-- event table
CREATE TABLE Event (
    no INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    playthrough_id_no TEXT NOT NULL,
    location_name TEXT NOT NULL,
    location_region TEXT NOT NULL,
    FOREIGN KEY (playthrough_id_no) REFERENCES Playthrough(id_no) ON DELETE RESTRICT,
    FOREIGN KEY (location_name, location_region) REFERENCES Location(name, region) ON DELETE RESTRICT
);

-- battle event table
CREATE TABLE Battle_Event (
    no INTEGER NOT NULL PRIMARY KEY,
    FOREIGN KEY (no) REFERENCES Event(no) ON DELETE RESTRICT
);

-- Pokemon Versions
INSERT INTO Version (name, generation) VALUES
    ("Red", 1),
    ("Blue", 1),
    ("Yellow", 1),
    ("Gold", 2),
    ("Silver", 2),
    ("Crystal", 2),
    ("Ruby", 3),
    ("Sapphire", 3),
    ("Emerald", 3),
    ("FireRed", 3),
    ("LeafGreen", 3),
    ("Diamond", 4),
    ("Pearl", 4),
    ("Platinum", 4),
    ("HeartGold", 4),
    ("SoulSilver", 4),
    ("Black", 5),
    ("White", 5),
    ("Black 2", 5),
    ("White 2", 5),
    ("X", 6),
    ("Y", 6),
    ("Omega Ruby", 6),
    ("Alpha Sapphire", 6),
    ("Sun", 7),
    ("Moon", 7),
    ("Ultra Sun", 7),
    ("Ultra Moon", 7),
    ("Let's Go Pikachu", 7),
    ("Let's Go Eevee", 7),
    ("Sword", 8),
    ("Shield", 8),
    ("Brilliant Diamond", 8),
    ("Shining Pearl", 8),
    ("Legends Arceus", 8),
    ("Scarlet", 9),
    ("Violet", 9);

-- Pokemon Regions
INSERT INTO Region (name) VALUES
    ("Kanto"),
    ("Johto"),
    ("Hoenn"),
    ("Sinnoh"),
    ("Unova"),
    ("Kalos"),
    ("Alola"),
    ("Galar"),
    ("Hisui"),
    ("Paldea");

-- Pokeballs
INSERT INTO Ball (name) VALUES
    ("Poké Ball"),
    ("Great Ball"),
    ("Ultra Ball"),
    ("Master Ball"),
    ("Safari Ball"),
    ("Fast Ball"),
    ("Level Ball"),
    ("Lure Ball"),
    ("Heavy Ball"),
    ("Love Ball"),
    ("Friend Ball"),
    ("Moon Ball"),
    ("Sport Ball"),
    ("Net Ball"),
    ("Dive Ball"),
    ("Nest Ball"),
    ("Repeat Ball"),
    ("Timer Ball"),
    ("Luxury Ball"),
    ("Premier Ball"),
    ("Dusk Ball"),
    ("Heal Ball"),
    ("Quick Ball"),
    ("Cherish Ball"),
    ("Park Ball"),
    ("Dream Ball"),
    ("Beast Ball"),
    ("Strange Ball"),
    ("Poké Ball (Hisui)"),
    ("Great Ball (Hisui)"),
    ("Ultra Ball (Hisui)"),
    ("Feather Ball"),
    ("Wing Ball"),
    ("Jet Ball"),
    ("Heavy Ball (Hisui)"),
    ("Leaden Ball"),
    ("Gigaton Ball"),
    ("Origin Ball");

-- Types
INSERT INTO Type (name, color) VALUES
    ("Normal", "#A8A77A"),
    ("Fire", "#EE8130"),
    ("Water", "#6390F0"),
    ("Electric", "#F7D02C"),
    ("Grass", "#7AC74C"),
    ("Ice", "#96D9D6"),
    ("Fighting", "#C22E28"),
    ("Poison", "#A33EA1"),
    ("Ground", "#E2BF65"),
    ("Flying", "#A98FF3"),
    ("Psychic", "#F95587"),
    ("Bug", "#A6B91A"),
    ("Rock", "#B6A136"),
    ("Ghost", "#735797"),
    ("Dragon", "#6F35FC"),
    ("Dark", "#705746"),
    ("Steel", "#B7B7CE"),
    ("Fairy", "#D685AD");