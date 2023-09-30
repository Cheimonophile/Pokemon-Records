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

-- Table for Battle Types
CREATE TABLE Battle_Type (
    name TEXT PRIMARY KEY NOT NULL
);

-- Table for Catch Types
CREATE TABLE Catch_Type (
    name TEXT PRIMARY KEY NOT NULL
);

-- Table for Items
CREATE TABLE Item (
    name TEXT PRIMARY KEY NOT NULL
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
    form TEXT,
    generation INTEGER NOT NULL,
    type1 TEXT NOT NULL,
    type2 TEXT,
    PRIMARY KEY (dex_no, form),
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
    slot INTEGER NOT NULL,
    nickname TEXT,
    caught_date TEXT NOT NULL,
    caught_location_name TEXT NOT NULL,
    caught_location_region TEXT NOT NULL,
    caught_species_dex_no INTEGER NOT NULL,
    caught_species_form TEXT,
    caught_level INTEGER NOT NULL,
    ball TEXT NOT NULL,
    gender TEXT NOT NULL,
    PRIMARY KEY (playthrough_id_no, slot),
    FOREIGN KEY (playthrough_id_no) REFERENCES Playthrough(id_no) ON DELETE RESTRICT,
    FOREIGN KEY (caught_location_name, caught_location_region) REFERENCES Location(name, region) ON DELETE RESTRICT,
    FOREIGN KEY (ball) REFERENCES Ball(name) ON DELETE RESTRICT,
    FOREIGN KEY (caught_species_dex_no, caught_species_form) REFERENCES Species(dex_no, form) ON DELETE RESTRICT,
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
CREATE TABLE Item_Event (
    no INTEGER NOT NULL PRIMARY KEY,
    item TEXT NOT NULL,
    FOREIGN KEY (no) REFERENCES Event(no) ON DELETE RESTRICT,
    FOREIGN KEY (item) REFERENCES Item(name) ON DELETE RESTRICT
);

-- catch event
CREATE TABLE Catch_Event (
    no INTEGER NOT NULL PRIMARY KEY,
    catch_type TEXT NOT NULL,
    FOREIGN KEY (no) REFERENCES Event(no) ON DELETE RESTRICT,
    FOREIGN KEY (catch_type) REFERENCES Catch_Type(name) ON DELETE RESTRICT
);

-- Team Member Changes
CREATE TABLE Team_Member_Change (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    team_member_playthrough_id_no TEXT NOT NULL,
    team_member_slot INTEGER NOT NULL,
    event_no INTEGER NOT NULL,
    level INTEGER,
    species_dex_no INTEGER,
    species_form TEXT,
    FOREIGN KEY (team_member_playthrough_id_no, team_member_slot) REFERENCES Team_Member(playthrough_id_no, slot) ON DELETE RESTRICT,
    FOREIGN KEY (event_no) REFERENCES Event(no) ON DELETE RESTRICT,
    CHECK (level >= 1 AND level <= 100),
    FOREIGN KEY (species_dex_no, species_form) REFERENCES Species(dex_no, form) ON DELETE RESTRICT
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

-- Battle Types
INSERT INTO Battle_Type (name) VALUES
    ("Single"),
    ("Double"),
    ("Triple"),
    ("Rotation"),
    ("Horde");


-- Catch Types
INSERT INTO Catch_Type (name) VALUES
    ("Gift"),
    ("Grass"),
    ("Cave"),
    ("Surf"),
    ("Old Rod"),
    ("Good Rod"),
    ("Super Rod"),
    ("Fossil"),
    ("Interact"),
    ("Hatch"),
    ("Rock Smash"),
    ("Headbutt"),
    -- ("Sweet Scent"),
    ("Long Grass"),
    ("Sand"),
    ("Seaweed"),
    ("Honey Tree"),
    ("Puddle"),
    ("Dark Grass"),
    ("Snow"),
    ("Ambush"),
    ("Soaring"),
    ("Shaking Tree"),
    ("Raid");