from src.events import *
from src.pkmn_constants import *

# Add Pokemon Types
types = {
    type: make_type(type)
    for type in TYPES
}

balls = {
    ball: make_ball(ball)
    for ball in BALLS
}

for event_type in EVENT_TYPES:
    make_event_type(event_type)

playthrough = make_playthrough(id_no="26852", name="Ben", version="Black", adventure_started=dt.date(2023, 6, 24))
unova = make_region(name="Unova")
nuvema_town = make_location(name="Nuvema Town", region=unova)
lillipup = receive_pokemon(
    playthrough=playthrough,
    slot=1,
    species=make_species(name="Lillipup", dex_no=506, type1=types["Normal"]),
    caught_date=dt.date(2023, 6, 24),
    caught_location=nuvema_town,
    caught_level=5,
    gender="M",
    ball="Poke Ball"
)
battle = make_battle(playthrough, nuvema_town, "PKMN Trainer Bianca")
battle = make_battle(playthrough, nuvema_town, "PKMN Trainer Cheren")
accumula_town = make_location(name="Accumula Town", region=unova)
battle = make_battle(playthrough, accumula_town, "PKMN Trainer N")
level_up(battle, lillipup, 7)
battle = make_battle(playthrough, rt2 := make_location("Route 2", unova), "Youngster Jimmy")
level_up(battle, lillipup, 8)
battle = make_battle(playthrough, rt2, "Lass Anna")
level_up(battle, lillipup, 9)
battle = make_battle(playthrough, rt2, "Youngster Roland")
level_up(battle, lillipup, 10)
battle = make_battle(playthrough, rt2, "PKMN Trainer Bianca")
dreamyard = make_location(name="Dreamyard", region=unova)
battle = make_battle(playthrough, dreamyard, "Lass Eri")
level_up(battle, lillipup, 11)
battle = make_battle(playthrough, dreamyard, "Youngster Joey")
level_up(battle, lillipup, 12)
pansear = receive_pokemon(
    playthrough=playthrough,
    slot=2,
    species=make_species(name="Pansear", dex_no=514, type1=types["Fire"]),
    caught_date=dt.date(2023, 6, 25),
    caught_location=dreamyard,
    caught_level=10,
    gender='M',
    ball="Poke Ball"
)
striaton_city = make_location(name="Striaton City", region=unova)
battle = make_battle(playthrough, striaton_city, "PKMN Trainer Cheren")
battle = make_battle(playthrough, striaton_city, "Waiter Maxwell")
level_up(battle, pansear, 11)
battle = make_battle(playthrough, striaton_city, "Waitress Tia")
battle = make_battle(playthrough, striaton_city, "Leader Cilan")
level_up(battle, lillipup, 13)
level_up(battle, pansear, 12)
battle = make_battle(playthrough, dreamyard, "Team Plasma Grunt")
battle = make_battle(playthrough, dreamyard, "Team Plasma Grunt")
level_up(battle, lillipup, 14)
rt3 = make_location(name="Route 3", region=unova)
battle = make_battle(playthrough, rt3, "Nursery Aid Autumn")
battle = make_battle(playthrough, rt3, "Preschooler Doyle")
level_up(battle, pansear, 13)
battle = make_battle(playthrough, rt3, 
    "Preschooler Wendy"
)
battle = make_battle(playthrough, rt3,
    "Preschooler Tully"
)
battle = make_battle(playthrough, rt3,
    "Twins Kumi & Amy"
)
battle = make_battle(playthrough, rt3,
    "PKMN Trainer Cheren"
)
level_up(battle,
    pansear,
    14)
battle = make_battle(playthrough, rt3,
    "PKMN Breeder Adelaide")
level_up(battle,
    lillipup,
    15)
wellspring_cave = make_location(name="Wellspring Cave", region=unova)
battle = make_battle(playthrough, wellspring_cave,
    "Team Plasma Grunt")
battle = make_battle(playthrough, wellspring_cave,
    "Team Plasma Grunt and Team Plasma Grunt with PKMN Trainer Cheren")
level_up(battle,
    pansear,
    15)
battle = make_battle(playthrough, rt3,
    "School Kid Al")
battle = make_battle(playthrough, rt3,
    "School Kid Marsha")
battle = make_battle(playthrough, rt3,
    "School Kid Gina")
level_up(battle,
    pansear,
    16)
battle = make_battle(playthrough, rt3,
    "School Kid Edgar")
pinwheel_forest = make_location(name="Pinwheel Forest", region=unova)
battle = make_battle(playthrough, pinwheel_forest,
    "Nurse Shery")
level_up(battle,
    lillipup,
    16)
herdier = evolve(battle,
    lillipup,
    dex_herdier := make_species(name="Herdier", dex_no=507, type1=types["Normal"]))
nacrene_city = make_location(name="Nacrene City", region=unova)
battle = make_battle(playthrough, nacrene_city,
    "PKMN Trainer N")
battle = make_battle(playthrough, pinwheel_forest,
    "Preschooler Juliet")
level_up(battle,
    herdier,
    17)
battle = make_battle(playthrough, pinwheel_forest,
    "Wild Tympole")
tympole = catch(
    battle,
    dex_tympole := make_species(name="Tympole", dex_no=535, type1=types["Water"]),
    slot=3,
    caught_date=dt.date(2023, 6, 26),
    caught_level=12,
    ball=balls['Net Ball'],
    gender='M')
battle = make_battle(playthrough, pinwheel_forest,
    "Preschooler Homer")
battle = make_battle(playthrough, pinwheel_forest,
    "Youngster Keita")
level_up(battle,
    herdier,
    18)
battle = make_battle(playthrough, pinwheel_forest,
    "Youngster Zachary")
battle = make_battle(playthrough, pinwheel_forest,
    "Battle Girl Lee")
level_up(battle,
    tympole,
    13)
battle = make_battle(playthrough, pinwheel_forest,
    "Black Belt Kentaro")
battle = make_battle(playthrough, nacrene_city,
    "School Kid Carter")
level_up(battle,
    tympole,
    14)
battle = make_battle(playthrough, nacrene_city,
    "Scientist Satomi")
level_up(battle,
    tympole,
    15)
battle = make_battle(playthrough, nacrene_city,
    "School Kid Lydia")
level_up(battle,
    pansear,
    17)
level_up(battle,
    herdier,
    19)
battle = make_battle(playthrough, nacrene_city,
    "Leader Lenora")
level_up(battle,
    herdier,
    20)
battle = make_battle(playthrough, pinwheel_forest,
    "Twins Mayo & May")
level_up(battle,
    tympole,
    16)
battle = make_battle(playthrough, pinwheel_forest,
    "Team Plasma Grunt")
battle = make_battle(playthrough, pinwheel_forest,
    "Team Plasma Grunt")
level_up(battle,
    tympole,
    17)
battle = make_battle(playthrough, pinwheel_forest,
    "PKMN Ranger Forrest")
level_up(battle,
    herdier,
    21)
battle = make_battle(playthrough, pinwheel_forest,
    "Youngster Nicholas")
level_up(battle,
    tympole,
    18)
battle = make_battle(playthrough, pinwheel_forest,
    "PKMN Ranger Audra")
battle = make_battle(playthrough, pinwheel_forest,
    "PKMN Ranger Irene")
battle = make_battle(playthrough, pinwheel_forest,
    "Team Plasma Grunt")
level_up(battle,
    pansear,
    18)
battle = make_battle(playthrough, pinwheel_forest,
    "PKMN Ranger Miguel")
battle = make_battle(playthrough, pinwheel_forest,
    "Team Plasma Grunt")
battle = make_battle(playthrough, pinwheel_forest,
    "Lass Eva")
level_up(battle,
    herdier,
    22)
battle = make_battle(playthrough, pinwheel_forest,
    "School Kid Sammy")
battle = make_battle(playthrough, pinwheel_forest,
    "School Kid Millie")
castelia_city = make_location(name="Castelia City", region=unova)
battle = make_battle(playthrough, castelia_city,
    "Clerk F Ingrid")
level_up(battle,
    pansear,
    19)
battle = make_battle(playthrough, castelia_city,
    "Clerk M Clemens")
level_up(battle,
    tympole,
    19)
battle = make_battle(playthrough, castelia_city,
    "Clerk F Alberta")
battle = make_battle(playthrough, castelia_city,
    "Scientist Randall")
battle = make_battle(playthrough, castelia_city,
    "Clerk M Warren")
level_up(battle,
    tympole,
    20)
level_up(battle,
    herdier,
    23)
battle = make_battle(playthrough, castelia_city,
    "Clerk M Ivan")
battle = make_battle(playthrough, castelia_city,
    "Scientist Samantha")
battle = make_battle(playthrough, castelia_city,
    "Scientist Steve")
level_up(battle,
    pansear,
    20)
battle = make_battle(playthrough, castelia_city,
    "Clerk M Wade")
battle = make_battle(playthrough, castelia_city,
    "Janitor Geoff")
level_up(battle,
    pansear,
    21)
level_up(battle,
    tympole,
    21)
battle = make_battle(playthrough, castelia_city,
    "Dancer Mickey")
battle = make_battle(playthrough, castelia_city,
    "Dancer Edmond")
route_4 = make_location(name="Route 4", region=unova)
battle = make_battle(playthrough, route_4,
    "Fisherman Hubert")
level_up(battle,
    pansear,
    22)
battle = make_battle(playthrough, route_4,
    "Fisherman Andrew")
level_up(battle,
    tympole,
    22)
level_up(battle,
    herdier,
    24)
battle = make_battle(playthrough, route_4,
    "Parasol Lady April")
level_up(battle,
    tympole,
    23)
battle = make_battle(playthrough, route_4,
    "Worker Gus")
battle = make_battle(playthrough, route_4,
    "Worker Shelby")
battle = make_battle(playthrough, castelia_city,
    "Dancer Raymond")
battle = make_battle(playthrough, castelia_city,
    "Team Plasma Grunt")
battle = make_battle(playthrough, castelia_city,
    "Harlequin Jack")
level_up(battle,
    pansear,
    23)
battle = make_battle(playthrough, castelia_city,
    "Harlequin Kerry")
battle = make_battle(playthrough, castelia_city,
    "Harlequin Rick")
battle = make_battle(playthrough, castelia_city,
    "Harlequin Louis")
level_up(battle,
    pansear,
    24)
battle = make_battle(playthrough, castelia_city,
    "Leader Burgh")
level_up(battle,
    tympole,
    24)
level_up(battle,
    pansear,
    25)
