-- Your SQL goes here

-- Versions
CREATE TABLE new_version (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    generation INTEGER NOT NULL
);
INSERT INTO new_version (name, generation)
  SELECT name, generation FROM Version;

-- Regions
CREATE TABLE new_region (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);
INSERT INTO new_region (name)
  SELECT name FROM Region;

-- Balls
CREATE TABLE new_ball (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);
INSERT INTO new_ball (name)
  SELECT name FROM Ball;

-- Types
CREATE TABLE new_type (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    color TEXT NOT NULL
);  
INSERT INTO new_type (name, color)
  SELECT name, color FROM Type;

-- Battle Types
CREATE TABLE new_battle_type (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);
INSERT INTO new_battle_type (name)
  SELECT name FROM Battle_Type;

-- Catch Types
CREATE TABLE new_catch_type (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);
INSERT INTO new_catch_type (name)
  SELECT name FROM Catch_Type;

-- Items
CREATE TABLE new_item (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);
INSERT INTO new_item (name)
  SELECT name FROM Item;

-- Locations
CREATE TABLE new_location (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    region_id INTEGER NOT NULL REFERENCES new_region(id)
);
INSERT INTO new_location (name, region_id)
  SELECT 
    Location.name AS name,
    new_region.id AS region_id
  FROM Location JOIN new_region ON Location.region = new_region.name;
CREATE VIEW new_location_with_region AS
  SELECT
    new_location.id AS id,
    new_location.name AS name,
    new_region.id AS region_id,
    new_region.name AS region_name
  FROM new_location
  LEFT JOIN new_region ON new_location.region_id = new_region.id;

-- Species
CREATE TABLE new_species (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  form TEXT,
  dex_no INTEGER NOT NULL,
  generation INTEGER NOT NULL,
  type1_id INTEGER NOT NULL REFERENCES new_type(id),
  type2_id INTEGER REFERENCES new_type(id),
  UNIQUE(name, form)
);
INSERT INTO new_species
  SELECT 
    NULL AS id,
    Species.name AS name,
    NULL AS form,
    Species.dex_no AS dex_no,
    Species.generation AS generation,
    new_type1.id AS type1_id,
    new_type2.id AS type2_id
  FROM Species
  LEFT JOIN new_type AS new_type1 ON Species.type1 = new_type1.name
  LEFT JOIN new_type AS new_type2 ON Species.type2 = new_type2.name;

-- Playthrough
CREATE TABLE new_playthrough (
  id_no TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  version_id INTEGER NOT NULL REFERENCES new_version(id),
  adventure_started TEXT NOT NULL
);
INSERT INTO new_playthrough
  SELECT 
    Playthrough.id_no AS id_no,
    Playthrough.name AS name,
    new_version.id AS version_id,
    Playthrough.adventure_started AS adventure_started
  FROM Playthrough
  JOIN new_version ON Playthrough.version = new_version.name;

-- Team Members
CREATE TABLE new_team_member (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  playthrough_id_no TEXT NOT NULL REFERENCES new_playthrough(id_no),
  slot INTEGER NOT NULL,
  nickname TEXT,
  caught_date TEXT NOT NULL,
  caught_location_id INTEGER REFERENCES new_location(id),
  caught_species_id INTEGER NOT NULL REFERENCES new_species(id),
  caught_level INTEGER NOT NULL,
  ball_id INTEGER NOT NULL REFERENCES new_ball(id),
  gender TEXT NOT NULL,
  CHECK (caught_level >= 1 AND caught_level <= 100),
  CHECK (gender IN ('M', 'F', 'N')),
  UNIQUE (playthrough_id_no, slot)
);
INSERT INTO new_team_member 
  SELECT
    Team_Member.id AS id,
    Team_Member.playthrough_id_no AS playthrough_id_no,
    Team_Member.slot AS slot,
    Team_Member.nickname AS nickname,
    Team_Member.caught_date AS caught_date,
    new_location_with_region.id AS caught_location_id,
    new_species.id AS caught_species_id,
    Team_Member.caught_level AS caught_level,
    new_ball.id AS ball_id,
    Team_Member.gender AS gender
  FROM Team_Member
  LEFT JOIN new_location_with_region ON 
    Team_Member.caught_location_name = new_location_with_region.name AND
    Team_Member.caught_location_region = new_location_with_region.region_name
  LEFT JOIN new_species ON Team_Member.caught_species_name = new_species.name
  LEFT JOIN new_ball ON Team_Member.ball = new_ball.name;

-- Trainer Classes
CREATE TABLE new_trainer_class (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL
);
INSERT INTO new_trainer_class (name)
  SELECT name FROM Trainer_Class;

-- Trainers
CREATE TABLE new_trainer (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  class_id INTEGER NOT NULL REFERENCES new_trainer_class(id)
);
INSERT INTO new_trainer
  SELECT
    NULL AS id,
    Trainer.name AS name,
    new_trainer_class.id AS class_id
  FROM Trainer
  LEFT JOIN new_trainer_class ON Trainer.class = new_trainer_class.name;
CREATE VIEW new_trainer_with_class AS
  SELECT
    new_trainer.id AS id,
    new_trainer.name AS name,
    new_trainer_class.id AS class_id,
    new_trainer_class.name AS class_name
  FROM new_trainer
  LEFT JOIN new_trainer_class ON new_trainer.class_id = new_trainer_class.id;

-- Event
CREATE TABLE new_event (
  no INTEGER PRIMARY KEY AUTOINCREMENT,
  playthrough_id_no TEXT NOT NULL REFERENCES new_playthrough(id_no),
  location_id INTEGER NOT NULL REFERENCES new_location(id),
  date TEXT NOT NULL
);
INSERT INTO new_event
  SELECT
    Event.no AS no,
    Event.playthrough_id_no AS playthrough_id_no,
    new_location_with_region.id AS location_id,
    DATE() AS date
  FROM Event
  LEFT JOIN new_location_with_region ON 
    Event.location_name = new_location_with_region.name AND
    Event.location_region = new_location_with_region.region_name;

-- Battle Event
CREATE TABLE new_battle_event (
  no INTEGER PRIMARY KEY REFERENCES new_event(no),
  battle_type_id INTEGER NOT NULL REFERENCES new_battle_type(id),
  opponent1_id INTEGER NOT NULL REFERENCES new_trainer(id),
  opponent2_id INTEGER REFERENCES new_trainer(id),
  partner_id INTEGER REFERENCES new_trainer(id),
  lost BOOLEAN NOT NULL,
  round INTEGER
);
INSERT INTO new_battle_event
  SELECT
    Battle_Event.no AS no,
    new_battle_type.id AS battle_type_id,
    new_opponent1.id AS opponent1_id,
    new_opponent2.id AS opponent2_id,
    new_partner.id AS partner_id,
    Battle_Event.lost AS lost,
    Battle_Event.round AS round
  FROM Battle_Event
  LEFT JOIN new_battle_type ON Battle_Event.battle_type = new_battle_type.name
  LEFT JOIN new_trainer_with_class AS new_opponent1 ON 
    Battle_Event.opponent1_name = new_opponent1.name AND
    Battle_Event.opponent1_class = new_opponent1.class_name
  LEFT JOIN new_trainer_with_class AS new_opponent2 ON
    Battle_Event.opponent2_name = new_opponent2.name AND
    Battle_Event.opponent2_class = new_opponent2.class_name
  LEFT JOIN new_trainer_with_class AS new_partner ON
    Battle_Event.partner_name = new_partner.name AND
    Battle_Event.partner_class = new_partner.class_name;

-- Item Event
CREATE TABLE new_item_event (
  no INTEGER PRIMARY KEY REFERENCES new_event(no),
  item_id INTEGER NOT NULL REFERENCES new_item(id)
);
INSERT INTO new_item_event
  SELECT
    Item_Event.no AS no,
    new_item.id AS item_id
  FROM Item_Event
  LEFT JOIN new_item ON Item_Event.item = new_item.name;

-- Catch Event
CREATE TABLE new_catch_event (
  no INTEGER PRIMARY KEY REFERENCES new_event(no),
  catch_type_id INTEGER NOT NULL REFERENCES new_catch_type(id),
  team_member_id INTEGER NOT NULL REFERENCES new_team_member(id)
);
INSERT INTO new_catch_event
  SELECT
    Catch_Event.no AS no,
    new_catch_type.id AS catch_type_id,
    Catch_Event.team_member_id AS team_member_id
  FROM Catch_Event
  LEFT JOIN new_catch_type ON Catch_Event.catch_type = new_catch_type.name;

-- Team Member Change
CREATE TABLE new_team_member_change (
  no INTEGER PRIMARY KEY AUTOINCREMENT,
  event_no INTEGER NOT NULL REFERENCES new_event(no),
  team_member_id INTEGER NOT NULL REFERENCES new_team_member(id),
  level INTEGER,
  species_id INTEGER REFERENCES new_species(id),
  CHECK (level >= 1 AND level <= 100)
);
INSERT INTO new_team_member_change
  SELECT
    Team_Member_Change.id AS no,
    Team_Member_Change.event_no AS event_no,
    Team_Member_Change.team_member_id AS team_member_id,
    Team_Member_Change.level AS level,
    new_species.id AS species_id
  FROM Team_Member_Change
  LEFT JOIN new_species ON Team_Member_Change.species_name = new_species.name;



-- TODO rename new tables



