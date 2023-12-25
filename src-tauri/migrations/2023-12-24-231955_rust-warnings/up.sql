-- Your SQL goes here


-- Rename old tables
ALTER TABLE Version RENAME TO old_version;
ALTER TABLE Region RENAME TO old_region;
ALTER TABLE Ball RENAME TO old_ball;
ALTER TABLE Type RENAME TO old_type;
ALTER TABLE Battle_Type RENAME TO old_battle_type;
ALTER TABLE Catch_Type RENAME TO old_catch_type;
ALTER TABLE Item RENAME TO old_item;
ALTER TABLE Location RENAME TO old_location;
ALTER TABLE Species RENAME TO old_species;
ALTER TABLE Playthrough RENAME TO old_playthrough;
ALTER TABLE Team_Member RENAME TO old_team_member;
ALTER TABLE Trainer_Class RENAME TO old_trainer_class;
ALTER TABLE Trainer RENAME TO old_trainer;
ALTER TABLE Event RENAME TO old_event;
ALTER TABLE Battle_Event RENAME TO old_battle_event;
ALTER TABLE Item_Event RENAME TO old_item_event;
ALTER TABLE Catch_Event RENAME TO old_catch_event;
ALTER TABLE Team_Member_Change RENAME TO old_team_member_change;


-- Versions
CREATE TABLE version (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    generation INTEGER NOT NULL,
    UNIQUE(name)
);
INSERT INTO version (name, generation)
  SELECT name, generation FROM old_version;

-- Regions
CREATE TABLE region (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    UNIQUE(name)
);
INSERT INTO region (name)
  SELECT name FROM old_region;

-- Balls
CREATE TABLE ball (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    UNIQUE(name)
);
INSERT INTO ball (name)
  SELECT name FROM old_ball;

-- Types
CREATE TABLE type (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    color TEXT NOT NULL,
    UNIQUE(name)
);  
INSERT INTO type (name, color)
  SELECT name, color FROM old_type;

-- Battle Types
CREATE TABLE battle_type (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    UNIQUE(name)
);
INSERT INTO battle_type (name)
  SELECT name FROM old_battle_type;

-- Catch Types
CREATE TABLE catch_type (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    detail Text,
    UNIQUE(name, detail)
);
INSERT INTO catch_type (name)
  SELECT name FROM old_catch_type;

-- Items
CREATE TABLE item (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    UNIQUE(name)
);
INSERT INTO item (name)
  SELECT name FROM old_item;

-- Locations
CREATE TABLE location (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    region_id INTEGER NOT NULL,
    FOREIGN KEY (region_id) REFERENCES region(id) ON DELETE RESTRICT,
    UNIQUE(name, region_id)
);
INSERT INTO location (name, region_id)
  SELECT 
    old_location.name AS name,
    region.id AS region_id
  FROM old_location LEFT JOIN region ON old_location.region = region.name;
CREATE VIEW location_with_region AS
  SELECT
    location.id AS id,
    location.name AS name,
    region.id AS region_id,
    region.name AS region_name
  FROM location
  LEFT JOIN region ON location.region_id = region.id;

-- Species
CREATE TABLE species (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  form TEXT,
  dex_no INTEGER NOT NULL,
  generation INTEGER NOT NULL,
  type1_id INTEGER NOT NULL,
  type2_id INTEGER,
  FOREIGN KEY (type1_id) REFERENCES type(id),
  FOREIGN KEY (type2_id) REFERENCES type(id),
  UNIQUE(name, form)
);
INSERT INTO species
  SELECT 
    NULL AS id,
    old_species.name AS name,
    NULL AS form,
    old_species.dex_no AS dex_no,
    old_species.generation AS generation,
    type1.id AS type1_id,
    type2.id AS type2_id
  FROM old_species
  LEFT JOIN type AS type1 ON old_species.type1 = type1.name
  LEFT JOIN type AS type2 ON old_species.type2 = type2.name;

-- Playthrough
CREATE TABLE playthrough (
  id_no TEXT NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  version_id INTEGER NOT NULL,
  adventure_started TEXT NOT NULL,
  FOREIGN KEY (version_id) REFERENCES version(id)
);
INSERT INTO playthrough
  SELECT 
    old_playthrough.id_no AS id_no,
    old_playthrough.name AS name,
    version.id AS version_id,
    old_playthrough.adventure_started AS adventure_started
  FROM old_playthrough
  LEFT JOIN version ON old_playthrough.version = version.name;

-- Team Members
CREATE TABLE team_member (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  playthrough_id_no TEXT NOT NULL,
  slot INTEGER NOT NULL,
  nickname TEXT,
  caught_date TEXT NOT NULL,
  caught_location_id INTEGER NOT NULL,
  caught_species_id INTEGER NOT NULL,
  caught_level INTEGER NOT NULL,
  ball_id INTEGER NOT NULL,
  gender TEXT NOT NULL,
  FOREIGN KEY (playthrough_id_no) REFERENCES playthrough(id_no),
  FOREIGN KEY (caught_location_id) REFERENCES location(id),
  FOREIGN KEY (caught_species_id) REFERENCES species(id),
  FOREIGN KEY (ball_id) REFERENCES ball(id),
  CHECK (caught_level >= 1 AND caught_level <= 100),
  CHECK (gender IN ('M', 'F', 'N')),
  UNIQUE (playthrough_id_no, slot)
);
INSERT INTO team_member 
  SELECT
    old_team_member.id AS id,
    old_team_member.playthrough_id_no AS playthrough_id_no,
    old_team_member.slot AS slot,
    old_team_member.nickname AS nickname,
    old_team_member.caught_date AS caught_date,
    location_with_region.id AS caught_location_id,
    species.id AS caught_species_id,
    old_team_member.caught_level AS caught_level,
    ball.id AS ball_id,
    old_team_member.gender AS gender
  FROM old_team_member
  LEFT JOIN location_with_region ON 
    old_team_member.caught_location_name = location_with_region.name AND
    old_team_member.caught_location_region = location_with_region.region_name
  LEFT JOIN species ON old_team_member.caught_species_name = species.name
  LEFT JOIN ball ON old_team_member.ball = ball.name;

-- Trainer Classes
CREATE TABLE trainer_class (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  UNIQUE(name)
);
INSERT INTO trainer_class (name)
  SELECT name FROM old_trainer_class;

-- Trainers
CREATE TABLE trainer (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  class_id INTEGER NOT NULL,
  FOREIGN KEY (class_id) REFERENCES trainer_class(id),
  UNIQUE(name, class_id)
);
INSERT INTO trainer
  SELECT
    NULL AS id,
    old_trainer.name AS name,
    trainer_class.id AS class_id
  FROM old_trainer
  LEFT JOIN trainer_class ON old_trainer.class = trainer_class.name;
CREATE VIEW trainer_with_class AS
  SELECT
    trainer.id AS id,
    trainer.name AS name,
    trainer_class.id AS class_id,
    trainer_class.name AS class_name
  FROM trainer
  LEFT JOIN trainer_class ON trainer.class_id = trainer_class.id;

-- Event
CREATE TABLE event (
  no INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  playthrough_id_no TEXT NOT NULL,
  location_id INTEGER NOT NULL,
  date TEXT NOT NULL,
  FOREIGN KEY (playthrough_id_no) REFERENCES playthrough(id_no),
  FOREIGN KEY (location_id) REFERENCES location(id)
);
INSERT INTO event
  SELECT
    old_event.no AS no,
    old_event.playthrough_id_no AS playthrough_id_no,
    location_with_region.id AS location_id,
    DATE() AS date
  FROM old_event
  LEFT JOIN location_with_region ON 
    old_event.location_name = location_with_region.name AND
    old_event.location_region = location_with_region.region_name;

-- Battle Event
CREATE TABLE battle_event (
  no INTEGER NOT NULL PRIMARY KEY,
  battle_type_id INTEGER NOT NULL,
  opponent1_id INTEGER NOT NULL,
  opponent2_id INTEGER,
  partner_id INTEGER,
  lost BOOLEAN NOT NULL,
  round INTEGER,
  FOREIGN KEY (no) REFERENCES event(no),
  FOREIGN KEY (battle_type_id) REFERENCES battle_type(id),
  FOREIGN KEY (opponent1_id) REFERENCES trainer(id),
  FOREIGN KEY (opponent2_id) REFERENCES trainer(id),
  FOREIGN KEY (partner_id) REFERENCES trainer(id)
);
INSERT INTO battle_event
  SELECT
    old_battle_event.no AS no,
    battle_type.id AS battle_type_id,
    opponent1.id AS opponent1_id,
    opponent2.id AS opponent2_id,
    partner.id AS partner_id,
    old_battle_event.lost AS lost,
    old_battle_event.round AS round
  FROM old_battle_event
  LEFT JOIN battle_type ON old_battle_event.battle_type = battle_type.name
  LEFT JOIN trainer_with_class AS opponent1 ON 
    old_battle_event.opponent1_name = opponent1.name AND
    old_battle_event.opponent1_class = opponent1.class_name
  LEFT JOIN trainer_with_class AS opponent2 ON
    old_battle_event.opponent2_name = opponent2.name AND
    old_battle_event.opponent2_class = opponent2.class_name
  LEFT JOIN trainer_with_class AS partner ON
    old_battle_event.partner_name = partner.name AND
    old_battle_event.partner_class = partner.class_name;

-- Item Event
CREATE TABLE item_event (
  no INTEGER NOT NULL PRIMARY KEY,
  item_id INTEGER NOT NULL,
  FOREIGN KEY (no) REFERENCES event(no),
  FOREIGN KEY (item_id) REFERENCES item(id)
);
INSERT INTO item_event
  SELECT
    old_item_event.no AS no,
    item.id AS item_id
  FROM old_item_event
  LEFT JOIN item ON old_item_event.item = item.name;

-- Catch Event
CREATE TABLE catch_event (
  no INTEGER NOT NULL PRIMARY KEY,
  catch_type_id INTEGER NOT NULL,
  team_member_id INTEGER NOT NULL,
  FOREIGN KEY (no) REFERENCES event(no),
  FOREIGN KEY (catch_type_id) REFERENCES catch_type(id),
  FOREIGN KEY (team_member_id) REFERENCES team_member(id)
);
INSERT INTO catch_event
  SELECT
    old_catch_event.no AS no,
    catch_type.id AS catch_type_id,
    old_catch_event.team_member_id AS team_member_id
  FROM old_catch_event
  LEFT JOIN catch_type ON old_catch_event.catch_type = catch_type.name;

-- Team Member Change
CREATE TABLE team_member_change (
  no INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  event_no INTEGER NOT NULL,
  team_member_id INTEGER NOT NULL,
  level INTEGER,
  species_id INTEGER,
  FOREIGN KEY (event_no) REFERENCES event(no),
  FOREIGN KEY (team_member_id) REFERENCES team_member(id),
  FOREIGN KEY (species_id) REFERENCES species(id),
  CHECK (level >= 1 AND level <= 100)
);
INSERT INTO team_member_change
  SELECT
    old_team_member_change.id AS no,
    old_team_member_change.event_no AS event_no,
    old_team_member_change.team_member_id AS team_member_id,
    old_team_member_change.level AS level,
    species.id AS species_id
  FROM old_team_member_change
  LEFT JOIN species ON old_team_member_change.species_name = species.name;



-- rename new tables
DROP TABLE old_team_member_change;
DROP TABLE old_Catch_event;
DROP TABLE old_item_event;
DROP TABLE old_battle_event;
DROP TABLE old_event;
DROP TABLE old_trainer;
DROP VIEW trainer_with_class;
DROP TABLE old_trainer_class;
DROP TABLE old_team_member;
DROP TABLE old_playthrough;
DROP TABLE old_species;
DROP TABLE old_location;
DROP VIEW location_with_region;
DROP TABLE old_item;
DROP TABLE old_catch_type;
DROP TABLE old_battle_type;
DROP TABLE old_type;
DROP TABLE old_ball;
DROP TABLE old_region;
DROP TABLE old_version;



