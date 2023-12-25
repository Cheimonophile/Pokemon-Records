-- This file should undo anything in `up.sql`


-- Rename old tables
ALTER TABLE version RENAME TO new_version;
ALTER TABLE region RENAME TO new_region;
ALTER TABLE ball RENAME TO new_ball;
ALTER TABLE type RENAME TO new_type;
ALTER TABLE battle_type RENAME TO new_battle_type;
ALTER TABLE catch_type RENAME TO new_catch_type;
ALTER TABLE item RENAME TO new_item;
ALTER TABLE location RENAME TO new_location;
ALTER TABLE species RENAME TO new_species;
ALTER TABLE playthrough RENAME TO new_playthrough;
ALTER TABLE team_member RENAME TO new_team_member;
ALTER TABLE trainer_class RENAME TO new_trainer_class;
ALTER TABLE trainer RENAME TO new_trainer;
ALTER TABLE event RENAME TO new_event;
ALTER TABLE battle_event RENAME TO new_battle_event;
ALTER TABLE item_event RENAME TO new_item_event;
ALTER TABLE catch_event RENAME TO new_catch_event;
ALTER TABLE team_member_change RENAME TO new_team_member_change;

-- Version
CREATE TABLE Version (
    name TEXT PRIMARY KEY NOT NULL,
    generation INTEGER NOT NULL
);
INSERT INTO Version
  SELECT
    name,
    generation
  FROM new_version;

-- Region
CREATE TABLE Region (
    name TEXT PRIMARY KEY NOT NULL
);
INSERT INTO Region
  SELECT
    name
  FROM new_region;

-- Ball
CREATE TABLE Ball (
    name TEXT PRIMARY KEY NOT NULL
);
INSERT INTO Ball
  SELECT
    name
  FROM new_ball;

-- Type
CREATE TABLE Type (
    name TEXT PRIMARY KEY NOT NULL,
    color TEXT NOT NULL
);
INSERT INTO Type
  SELECT
    name,
    color
  FROM new_type;

-- Battle Type
CREATE TABLE Battle_Type (
    name TEXT PRIMARY KEY NOT NULL
);
INSERT INTO Battle_Type
  SELECT
    name
  FROM new_battle_type;

-- Catch Type
CREATE TABLE Catch_Type (
    name TEXT PRIMARY KEY NOT NULL
);
INSERT INTO Catch_Type
  SELECT
    CASE WHEN detail IS NULL THEN name ELSE name || ' (' || detail || ')' END AS name
  FROM new_catch_type;

-- Item
CREATE TABLE Item (
    name TEXT PRIMARY KEY NOT NULL
);
INSERT INTO Item
  SELECT
    name
  FROM new_item;

-- Location
CREATE TABLE Location (
    name TEXT NOT NULL,
    region TEXT NOT NULL,
    PRIMARY KEY (name, region),
    FOREIGN KEY (region) REFERENCES Region(name) ON DELETE RESTRICT
);
INSERT INTO Location
  SELECT
    new_location.name AS name,
    new_region.name AS region
  FROM new_location
  LEFT JOIN new_region ON new_location.region_id = new_region.id;

-- Species
CREATE TABLE Species (
    name TEXT NOT NULL,
    dex_no INTEGER NOT NULL,
    generation INTEGER NOT NULL,
    type1 TEXT NOT NULL,
    type2 TEXT,
    PRIMARY KEY (name),
    FOREIGN KEY (type1) REFERENCES Type(name) ON DELETE RESTRICT,
    FOREIGN KEY (type2) REFERENCES Type(name) ON DELETE RESTRICT
);
INSERT INTO Species
  SELECT
    CASE WHEN new_species.form IS NULL
      THEN new_species.name
      ELSE new_species.name || " (" || new_species.form || ")" 
    END AS name,
    new_species.dex_no AS dex_no,
    new_species.generation AS generation,
    new_type1.name AS type1,
    new_type2.name AS type2
  FROM new_species
  LEFT JOIN new_type AS new_type1 ON new_species.type1_id = new_type1.id
  LEFT JOIN new_type AS new_type2 ON new_species.type2_id = new_type2.id;

-- Playthrough
CREATE TABLE Playthrough (
    id_no TEXT NOT NULL,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    adventure_started TEXT NOT NULL,
    PRIMARY KEY (id_no),
    FOREIGN KEY (version) REFERENCES Version(name) ON DELETE RESTRICT
);
INSERT INTO Playthrough
  SELECT
    new_playthrough.id_no AS id_no,
    new_playthrough.name AS name,
    new_version.name AS version,
    new_playthrough.adventure_started AS adventure_started
  FROM new_playthrough
  LEFT JOIN new_version ON new_playthrough.version_id = new_version.id;

-- Team Member
CREATE TABLE Team_Member (
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
INSERT INTO Team_Member
  SELECT
    new_team_member.id AS id,
    new_team_member.playthrough_id_no AS playthrough_id_no,
    new_team_member.slot AS slot,
    new_team_member.nickname AS nickname,
    new_team_member.caught_date AS caught_date,
    new_location.name AS caught_location_name,
    new_region.name AS caught_location_region,
    CASE WHEN new_species.form IS NULL
      THEN new_species.name
      ELSE new_species.name || " (" || new_species.form || ")" 
    END AS caught_species_name,
    new_team_member.caught_level AS caught_level,
    new_ball.name AS ball,
    new_team_member.gender AS gender
  FROM new_team_member
  LEFT JOIN new_location ON new_team_member.caught_location_id = new_location.id
  LEFT JOIN new_region ON new_location.region_id = new_region.id
  LEFT JOIN new_species ON new_team_member.caught_species_id = new_species.id
  LEFT JOIN new_ball ON new_team_member.ball_id = new_ball.id;

-- Trainer Class
CREATE TABLE Trainer_Class (
    name TEXT PRIMARY KEY NOT NULL
);
INSERT INTO Trainer_Class
  SELECT
    name
  FROM new_trainer_class;

-- Trainer
CREATE TABLE Trainer (
    name TEXT NOT NULL,
    class TEXT NOT NULL,
    PRIMARY KEY (name, class),
    FOREIGN KEY (class) REFERENCES Trainer_Class(name) ON DELETE RESTRICT
);
INSERT INTO Trainer
  SELECT
    new_trainer.name AS name,
    new_trainer_class.name AS class
  FROM new_trainer
  LEFT JOIN new_trainer_class ON new_trainer.class_id = new_trainer_class.id;

-- Event
CREATE TABLE Event (
    no INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    playthrough_id_no TEXT NOT NULL,
    location_name TEXT NOT NULL,
    location_region TEXT NOT NULL,
    FOREIGN KEY (playthrough_id_no) REFERENCES Playthrough(id_no) ON DELETE RESTRICT,
    FOREIGN KEY (location_name, location_region) REFERENCES Location(name, region) ON DELETE RESTRICT
);
INSERT INTO Event
  SELECT
    new_event.no AS no,
    new_event.playthrough_id_no AS playthrough_id_no,
    new_location.name AS location_name,
    new_region.name AS location_region
  FROM new_event
  LEFT JOIN new_location ON new_event.location_id = new_location.id
  LEFT JOIN new_region ON new_location.region_id = new_region.id;

-- Battle Event
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
INSERT INTO Battle_Event
  SELECT
    new_battle_event.no AS no,
    new_battle_type.name AS battle_type,
    new_opponent1.name AS opponent1_name,
    new_opponent1_class.name AS opponent1_class,
    new_opponent2.name AS opponent2_name,
    new_opponent2_class.name AS opponent2_class,
    new_partner.name AS partner_name,
    new_partner_class.name AS partner_class,
    new_battle_event.round AS round,
    new_battle_event.lost AS lost
  FROM new_battle_event
  LEFT JOIN new_battle_type ON new_battle_event.battle_type_id = new_battle_type.id
  LEFT JOIN new_trainer AS new_opponent1 ON new_battle_event.opponent1_id = new_opponent1.id
  LEFT JOIN new_trainer_class AS new_opponent1_class ON new_opponent1.class_id = new_opponent1_class.id
  LEFT JOIN new_trainer AS new_opponent2 ON new_battle_event.opponent2_id = new_opponent2.id
  LEFT JOIN new_trainer_class AS new_opponent2_class ON new_opponent2.class_id = new_opponent2_class.id
  LEFT JOIN new_trainer AS new_partner ON new_battle_event.partner_id = new_partner.id
  LEFT JOIN new_trainer_class AS new_partner_class ON new_partner.class_id = new_partner_class.id;

-- Item Event
CREATE TABLE Item_Event (
    no INTEGER NOT NULL PRIMARY KEY,
    item TEXT NOT NULL,
    FOREIGN KEY (no) REFERENCES Event(no) ON DELETE RESTRICT,
    FOREIGN KEY (item) REFERENCES Item(name) ON DELETE RESTRICT
);
INSERT INTO Item_Event
  SELECT
    new_item_event.no AS no,
    new_item.name AS item
  FROM new_item_event
  LEFT JOIN new_item ON new_item_event.item_id = new_item.id;

-- Catch Event
CREATE TABLE Catch_Event (
    no INTEGER NOT NULL PRIMARY KEY,
    catch_type TEXT NOT NULL,
    team_member_id INTEGER NOT NULL,
    FOREIGN KEY (no) REFERENCES Event(no) ON DELETE RESTRICT,
    FOREIGN KEY (catch_type) REFERENCES Catch_Type(name) ON DELETE RESTRICT,
    FOREIGN KEY (team_member_id) REFERENCES Team_Member(id) ON DELETE RESTRICT
);
INSERT INTO Catch_Event
  SELECT
    new_catch_event.no AS no,
    new_catch_type.name AS catch_type,
    new_catch_event.team_member_id AS team_member_id
  FROM new_catch_event
  LEFT JOIN new_catch_type ON new_catch_event.catch_type_id = new_catch_type.id;

-- Team Member Change
CREATE TABLE Team_Member_Change (
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
INSERT INTO Team_Member_Change
  SELECT
    new_team_member_change.no AS id,
    new_team_member_change.team_member_id AS team_member_id,
    new_team_member_change.event_no AS event_no,
    new_team_member_change.level AS level,
    CASE WHEN new_species.form IS NULL
      THEN new_species.name
      ELSE new_species.name || " (" || new_species.form || ")" 
    END AS species_name
  FROM new_team_member_change
  LEFT JOIN new_species ON new_team_member_change.species_id = new_species.id;



-- Drop new tables
DROP TABLE new_version;
DROP TABLE new_region;
DROP TABLE new_ball;
DROP TABLE new_type;
DROP TABLE new_battle_type;
DROP TABLE new_catch_type;
DROP TABLE new_item;
DROP TABLE new_location;
DROP TABLE new_species;
DROP TABLE new_playthrough;
DROP TABLE new_team_member;
DROP TABLE new_trainer_class;
DROP TABLE new_trainer;
DROP TABLE new_event;
DROP TABLE new_battle_event;
DROP TABLE new_item_event;
DROP TABLE new_catch_event;
DROP TABLE new_team_member_change;
