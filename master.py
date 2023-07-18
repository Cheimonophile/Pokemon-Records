from src.events import *
from src.pkmn_constants import *


playthrough = make_playthrough(id_no="26852", name="Ben", version="Black", adventure_started=dt.date(2023, 6, 24))
nuvema_town = make_location(name="Nuvema Town", region="Unova")
lillipup = receive_pokemon(playthrough,1,
    "Lillipup",
    dex_no=506,
    type1="Normal",
    caught_date=dt.date(2023, 6, 24),
    caught_location=nuvema_town,
    caught_level=5,
    gender="M",
    ball="Poke Ball",
)
battle = make_battle(playthrough, nuvema_town, "PKMN Trainer Bianca")
battle = make_battle(playthrough, nuvema_town, "PKMN Trainer Cheren")
accumula_town = make_location(name="Accumula Town", region="Unova")
battle = make_battle(playthrough, accumula_town, "PKMN Trainer N")
level_up(battle, lillipup, 7)
battle = make_battle(playthrough, rt2 := make_location("Route 2", "Unova"), "Youngster Jimmy")
level_up(battle, lillipup, 8)
battle = make_battle(playthrough, rt2, "Lass Anna")
level_up(battle, lillipup, 9)
battle = make_battle(playthrough, rt2, "Youngster Roland")
level_up(battle, lillipup, 10)
battle = make_battle(playthrough, rt2, "PKMN Trainer Bianca")
dreamyard = make_location(name="Dreamyard", region="Unova")
battle = make_battle(playthrough, dreamyard, "Lass Eri")
level_up(battle, lillipup, 11)
battle = make_battle(playthrough, dreamyard, "Youngster Joey")
level_up(battle, lillipup, 12)
pansear = receive_pokemon(playthrough,
    slot=2,
    species="Pansear",
    dex_no=513,
    type1="Fire",
    caught_date=dt.date(2023, 6, 25),
    caught_location=dreamyard,
    caught_level=10,
    gender='M',
    ball="Poke Ball"
)
striaton_city = make_location(name="Striaton City", region="Unova")
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
rt3 = make_location(name="Route 3", region="Unova")
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
wellspring_cave = make_location(name="Wellspring Cave", region="Unova")
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
pinwheel_forest = make_location(name="Pinwheel Forest", region="Unova")
battle = make_battle(playthrough, pinwheel_forest,
    "Nurse Shery")
level_up(battle,
    lillipup,
    16)
herdier = evolve(battle,
    lillipup,
    "Herdier",
    dex_no=507,
    type1="Normal")
nacrene_city = make_location(name="Nacrene City", region="Unova")
battle = make_battle(playthrough, nacrene_city,
    "PKMN Trainer N")
battle = make_battle(playthrough, pinwheel_forest,
    "Preschooler Juliet")
level_up(battle,
    herdier,
    17)
battle = make_battle(playthrough, pinwheel_forest,
    "Wild Tympole")
tympole = catch(battle,
    slot=3,
    species="Tympole",
    dex_no=535,
    type1="Water",
    caught_date=dt.date(2023, 6, 26),
    caught_level=12,
    ball="Net Ball",
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
castelia_city = make_location(name="Castelia City", region="Unova")
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
route_4 = make_location(name="Route 4", region="Unova")
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
castelia_gate = make_location(name="Castelia Gate", region="Unova")
battle = make_battle(playthrough, castelia_gate,
    "PKMN Trainer Bianca")
level_up(battle,
    herdier,
    25)
battle = make_battle(playthrough, route_4,
    "PKMN Trainer Cheren", lost=True)
battle = make_battle(playthrough, route_4,
    "PKMN Trainer Cheren")
level_up(battle,
    pansear,
    26)
battle = make_battle(playthrough, route_4,
    "Backpacker Keane")
level_up(battle,
    tympole,
    25)
palpitoad = evolve(battle,
    tympole,
    "Palpitoad",
    dex_no=536,
    type1="Water",
    type2="Ground")
battle = make_battle(playthrough, route_4,
    "Backpacker Anna")
battle = make_battle(playthrough, route_4,
    "Backpacker Jill")
battle = make_battle(playthrough, route_4,
    "Backpacker Waylon")
battle = make_battle(playthrough, route_4,
    "Worker Scott")
battle = make_battle(playthrough, route_4,
    "Worker Zack")
desert_resort = make_location(name="Desert Resort", region="Unova")
battle = make_battle(playthrough, desert_resort,
    "Doctor Jerry")
battle = make_battle(playthrough, desert_resort,
    "Backpacker Kelsey")
level_up(battle,
    palpitoad,
    26)
battle = make_battle(playthrough, desert_resort,
    "Backpacker Liz")
battle = make_battle(playthrough, desert_resort,
    "Psychic Cybil")
level_up(battle,
    herdier,
    26)
battle = make_battle(playthrough, desert_resort,
    "Backpacker Nate")
battle = make_battle(playthrough, desert_resort,
    "Backpacker Elaine")
battle = make_battle(playthrough, desert_resort,
    "Psychic Gaven")
level_up(battle,
    herdier,
    27)
battle = make_battle(playthrough, desert_resort,
    "Psychic Low")
battle = make_battle(playthrough, desert_resort,
    "PKMN Ranger Mylene")
battle = make_battle(playthrough, desert_resort,
    "PKMN Ranger Jaden")
level_up(battle,
    palpitoad,
    27)
relic_castle = make_location(name="Relic Castle", region="Unova")
battle = make_battle(playthrough, relic_castle,
    "Psychic Perry")
battle = make_battle(playthrough, relic_castle,
    "Psychic Dua")
level_up(battle,
    pansear,
    27)
battle = make_battle(playthrough, route_4,
    "Backpacker Jerome")
nimbasa_city = make_location(name="Nimbasa City", region="Unova")
battle = make_battle(playthrough, nimbasa_city,
    "Team Plasma Grunt")
battle = make_battle(playthrough, nimbasa_city,
    "Linebacker Dan")
battle = make_battle(playthrough, nimbasa_city,
    "Hoopster Bobby")
level_up(battle,
    pansear,
    28)
battle = make_battle(playthrough, nimbasa_city,
    "PKMN Trainer N")
level_up(battle,
    herdier,
    28)
route_16 = make_location(name="Route 16", region="Unova")
battle = make_battle(playthrough, route_16,
    "Policeman Daniel")
battle = make_battle(playthrough, route_16,
    "Cyclist Krissa")
level_up(battle,
    palpitoad,
    28)
battle = make_battle(playthrough, route_16,
    "Backpacker Peter")
battle = make_battle(playthrough, route_16,
    "Cyclist Hector")
battle = make_battle(playthrough, route_16,
    "Backpacker Stephen")
level_up(battle,
    palpitoad,
    29)
route_5 = make_location(name="Route 5", region="Unova")
battle = make_battle(playthrough, route_5,
    "Backpacker Lois")
battle = make_battle(playthrough, route_5,
    "Backpacker Michael")
battle = make_battle(playthrough, route_5,
    "Baker Jenn")
battle = make_battle(playthrough, route_5,
    "Harlequin Paul")
level_up(battle,
    pansear,
    29)
battle = make_battle(playthrough, route_5,
    "Musician Preston")
battle = make_battle(playthrough, route_5,
    "Dancer Brian")
level_up(battle,
    herdier,
    29)
battle = make_battle(playthrough, route_5,
    "Artist Horton")
battle = make_battle(playthrough, nimbasa_city,
    "Lady Magnolia")
battle = make_battle(playthrough, nimbasa_city,
    "Rich Boy Cody")
level_up(battle,
    palpitoad,
    30)
battle = make_battle(playthrough, nimbasa_city,
    "Rich Boy Rolan")
battle = make_battle(playthrough, nimbasa_city,
    "Lady Colette")
battle = make_battle(playthrough, nimbasa_city,
    "Leader Elesa")
level_up(battle,
    palpitoad,
    31)
archen = revive_fossil(playthrough,
    "Plume Fossil",
    slot=4,
    species="Archen",
    dex_no=566,
    type1="Rock",
    type2="Flying",
    gender='M',
    caught_date=dt.date(2023, 7,2),
    caught_location=nacrene_city,
    caught_level=25,
    ball='Poke Ball')
battle = make_battle(playthrough, route_5,
    "PKMN Trainer Cheren")
level_up(battle,
    herdier,
    30)
level_up(battle,
    palpitoad,
    32)
battle = make_battle(playthrough, route_5,
    "Preschooler Sarah and Preschooler Billy with PKMN Trainer Cheren") # double battle
level_up(battle,
    archen,
    26)
cold_storage = make_location(name="Cold Storage", region="Unova")
battle = make_battle(playthrough, cold_storage,
    "Youngster Kenneth")
battle = make_battle(playthrough, cold_storage,
    "Youngster Albert")
battle = make_battle(playthrough, cold_storage,
    "Worker Eddie")
level_up(battle,
    archen,
    27)
battle = make_battle(playthrough, cold_storage,
    "Worker Victor")
level_up(battle,
    pansear,
    30)
battle = make_battle(playthrough, cold_storage,
    "Worker Glenn")
battle = make_battle(playthrough, cold_storage,
    "Worker Filipe")
battle = make_battle(playthrough, cold_storage,
    "Worker Patton")
battle = make_battle(playthrough, cold_storage,
    "Worker Ryan")
level_up(battle,
    archen,
    28)
battle = make_battle(playthrough, cold_storage,
    "Team Plasma Grunt")
battle = make_battle(playthrough, cold_storage,
    "Team Plasma Grunt")
battle = make_battle(playthrough, cold_storage,
    "Team Plasma Grunt")
battle = make_battle(playthrough, cold_storage,
    "Team Plasma Grunt")
route_6 = make_location(name="Route 6", region="Unova")
battle = make_battle(playthrough, route_6,
    "Scientist William")
battle = make_battle(playthrough, route_6,
    "PKMN Ranger Shanti")
level_up(battle,
    herdier,
    31)
level_up(battle,
    archen,
    29)
battle = make_battle(playthrough, route_6,
    "Parasol Lady Nicole")
battle = make_battle(playthrough, route_6,
    "Scientist Ron")
battle = make_battle(playthrough, route_6,
    "Scientist Maria")
battle = make_battle(playthrough, route_6,
    "Parasol Lady Tihana")
battle = make_battle(playthrough, route_6,
    "PKMN Ranger Richard")
level_up(battle,
    pansear,
    31)
level_up(battle,
    archen,
    30)
driftveil_city = make_location(name="Driftveil City", region="Unova")
battle = make_battle(playthrough, driftveil_city,
    "Worker Felix")
battle = make_battle(playthrough, driftveil_city,
    "Worker Sterling")
battle = make_battle(playthrough, driftveil_city,
    "Worker Don")
battle = make_battle(playthrough, driftveil_city,
    "Clerk M Isaac")
level_up(battle,
    archen,
    31)
level_up(battle,
    palpitoad,
    33)
battle = make_battle(playthrough, driftveil_city,
    "Clerk F Katie")
battle = make_battle(playthrough, driftveil_city,
    "Leader Clay")
battle = make_battle(playthrough, driftveil_city,
    "PKMN Trainer Bianca")
level_up(battle,
    archen,
    32)
chargestone_cave = make_location(name="Chargestone Cave", region="Unova")
battle = make_battle(playthrough, chargestone_cave,
    "Ace Trainer Jared")
battle = make_battle(playthrough, chargestone_cave,
    "Scientist Ronald")
battle = make_battle(playthrough, chargestone_cave,
    "Hiker Hardy")
level_up(battle,
    pansear,
    32)
battle = make_battle(playthrough, chargestone_cave,
    "Scientist Naoko")
battle = make_battle(playthrough, chargestone_cave,
    "Doctor Wayne")
level_up(battle,
    archen,
    33)
battle = make_battle(playthrough, chargestone_cave,
    "Team Plasma Grunt")
level_up(battle,
    herdier,
    32)
stoutland = evolve(battle,
    herdier,
    "Stoutland",
    dex_no=508,
    type1="Normal")
battle = make_battle(playthrough, chargestone_cave,
    "Team Plasma Grunt")
battle = make_battle(playthrough, chargestone_cave,
    "Team Plasma Grunt")
battle = make_battle(playthrough, chargestone_cave,
    "Team Plasma Grunt")
battle = make_battle(playthrough, chargestone_cave,
    "Team Plasma Grunt")
level_up(battle,
    stoutland,
    33)
battle = make_battle(playthrough, chargestone_cave,
    "Team Plasma Grunt")
level_up(battle,
    archen,
    34)
battle = make_battle(playthrough, chargestone_cave,
    "Team Plasma Grunt")
battle = make_battle(playthrough, chargestone_cave,
    "Ace Trainer Allison")
level_up(battle,
    palpitoad,
    34)
battle = make_battle(playthrough, chargestone_cave,
    "Ace Trainer Stella")
level_up(battle,
    pansear,
    33)
level_up(battle,
    stoutland,
    34)
battle = make_battle(playthrough, chargestone_cave,
    "Scientist Orville")
battle = make_battle(playthrough, chargestone_cave,
    "Ace Trainer Corky")
level_up(battle,
    pansear,
    34)
battle = make_battle(playthrough, chargestone_cave,
    "PKMN Trainer N")
mistralton_city = make_location(name="Mistralton City", region="Unova")
route_7 = make_location(name="Route 7", region="Unova")
battle = make_battle(playthrough, route_7,
    "Youngster Mikey")
level_up(battle,
    pansear,
    35)
battle = make_battle(playthrough, route_7,
    "Youngster Parker")
battle = make_battle(playthrough, route_7,
    "Backpacker Terrance")
battle = make_battle(playthrough, route_7,
    "Ace Trainer Elmer",
    battle_type='Rotation')
level_up(battle,
    archen,
    35)
level_up(battle,
    stoutland,
    35)
level_up(battle,
    palpitoad,
    35)
battle = make_battle(playthrough, route_7,
    "Backpacker Ruth")
battle = make_battle(playthrough, route_7,
    "PKMN Ranger Mary")
level_up(battle,
    archen,
    36)
battle = make_battle(playthrough, route_7,
    "PKMN Ranger Pedro")
battle = make_battle(playthrough, route_7,
    "Harlequin Pat")
level_up(battle,
    pansear,
    36)
battle = make_battle(playthrough, route_7,
    "Harlequin Ian")
level_up(battle,
    stoutland,
    36)
twist_mountain = make_location(name="Twist Mountain", region="Unova")
battle = make_battle(playthrough, twist_mountain,
    "Hiker Terrell")
level_up(battle,
    palpitoad,
    36)
seismitoad = evolve(battle,
    palpitoad,
    "Seismitoad",
    dex_no=537,
    type1="Water",
    type2="Ground")
celestial_tower = make_location(name="Celestial Tower", region="Unova")
battle = make_battle(playthrough, celestial_tower,
    "Psychic Doreen")
level_up(battle,
    seismitoad,
    37)
battle = make_battle(playthrough, celestial_tower,
    "Lass Kara")
battle = make_battle(playthrough, celestial_tower,
    "Pokéfan Jude")
battle = make_battle(playthrough, celestial_tower,
    "Pokéfan Georgia")
battle = make_battle(playthrough, celestial_tower,
    "Psychic Belle")
level_up(battle,
    archen,
    37)
archeops = evolve(battle,
    archen,
    "Archeops",
    dex_no=567,
    type1="Rock",
    type2="Flying")
battle = make_battle(playthrough, celestial_tower,
    "Psychic Lin")
battle = make_battle(playthrough, celestial_tower,
    "Psychic Micki")
battle = make_battle(playthrough, celestial_tower,
    "Psychic Bryce")
battle = make_battle(playthrough, celestial_tower,
    "Nurse Sachiko")
level_up(battle,
    pansear,
    37)
battle = make_battle(playthrough, celestial_tower,
    "Ace Trainer Beckett")
battle = make_battle(playthrough, celestial_tower,
    "Ace Trainer Kassandra")
level_up(battle,
    stoutland,
    37)
battle = make_battle(playthrough, mistralton_city,
    "Worker Cliff")
level_up(battle,
    archeops,
    38)
battle = make_battle(playthrough, mistralton_city,
    "Worker Brady")
battle = make_battle(playthrough, mistralton_city,
    "Pilot Ted")
battle = make_battle(playthrough, mistralton_city,
    "Pilot Chase")
level_up(battle,
    archeops,
    39)
battle = make_battle(playthrough, mistralton_city,
    "Worker Arnold")
battle = make_battle(playthrough, mistralton_city,
    "Leader Skyla")
level_up(battle,
    archeops,
    40)




