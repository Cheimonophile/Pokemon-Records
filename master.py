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
    "Twins Kumi & Amy",
    battle_type="Double"
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
    "Team Plasma Grunt and Team Plasma Grunt with PKMN Trainer Cheren",
    battle_type="Double")
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
    "Twins Mayo & May",
    battle_type="Double")
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
    "Preschooler Sarah and Preschooler Billy with PKMN Trainer Cheren",
    battle_type="Double")
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
battle = make_battle(playthrough, twist_mountain,
    "PKMN Trainer Cheren")
level_up(battle,
    seismitoad,
    38)
route_1 = make_location(name="Route 1", region="Unova")
battle = make_battle(playthrough, route_1,
    "PKMN Ranger Brenda")
level_up(battle,
    stoutland,
    38)
battle = make_battle(playthrough, route_1,
    "Fisherman Sean")
battle = make_battle(playthrough, route_1,
    "Ranger Claude")
route_17 = make_location(name="Route 17", region="Unova")
battle = make_battle(playthrough, route_17,
    "Fisherman Lydon")
level_up(battle,
    pansear,
    38)
level_up(battle,
    stoutland,
    39)
battle = make_battle(playthrough, route_17,
    "Swimmer F Joyce")
battle = make_battle(playthrough, route_17,
    "Swimmer M Wright")
level_up(battle,
    seismitoad,
    39)
battle = make_battle(playthrough, route_17,
    "Hiker Jeremiah")
level_up(battle,
    seismitoad,
    40)
battle = make_battle(playthrough, route_17,
    "Backpacker Kumiko")
battle = make_battle(playthrough, route_17,
    "Backpacker Sam")
level_up(battle,
    pansear,
    39)
battle = make_battle(playthrough, route_17,
    "Veteran Ray")
p2_laboratory = make_location(name="P2 Laboratory", region="Unova")
battle = make_battle(playthrough, route_17,
    "Scientist Nathan")
battle = make_battle(playthrough, route_17,
    "Swimmer M Berke")
level_up(battle,
    stoutland,
    40)
battle = make_battle(playthrough, route_17,
    "Swimmer F Kelsey")
battle = make_battle(playthrough, route_17,
    "Battle Girl Hillary")
mistralton_cave = make_location(name="Mistralton Cave", region="Unova")
battle = make_battle(playthrough, mistralton_cave,
    "Hiker Hugh")
level_up(battle,
    pansear,
    40)
battle = make_battle(playthrough, mistralton_cave,
    "Hiker Clarke")
battle = make_battle(playthrough, twist_mountain,
    "Hiker Darrell")
level_up(battle,
    seismitoad,
    41)
battle = make_battle(playthrough, twist_mountain,
    "Ace Trainer Caroll")
level_up(battle,
    pansear,
    41)
battle = make_battle(playthrough, twist_mountain,
    "Battle Girl Sharon")
level_up(battle,
    archeops,
    41)
battle = make_battle(playthrough, twist_mountain,
    "Worker Brand")
battle = make_battle(playthrough, twist_mountain,
    "Worker Heath")
battle = make_battle(playthrough, twist_mountain,
    "Worker Rob")
level_up(battle,
    seismitoad,
    42)
battle = make_battle(playthrough, twist_mountain,
    "Worker Cairn")
battle = make_battle(playthrough, twist_mountain,
    "Doctor Hank")
level_up(battle,
    stoutland,
    41)
battle = make_battle(playthrough, twist_mountain,
    "Ace Trainer Jordan")
level_up(battle,
    stoutland,
    42)
dragonspiral_tower = make_location(name="Dragonspiral Tower", region="Unova")
battle = make_battle(playthrough, dragonspiral_tower,
    "Wild Mienfoo")
mienfoo = catch(battle,
    slot=5,
    species="Mienfoo",
    dex_no=619,
    type1="Fighting",
    caught_date=dt.date(2023, 8, 22),
    caught_level=32,
    ball="Great Ball",
    gender='M')
route_8 = make_location(name="Route 8", region="Unova")
battle = make_battle(playthrough, route_8,
    "PKMN Ranger Lewis")
battle = make_battle(playthrough, route_8,
    "Parasol Lady Melita")
battle = make_battle(playthrough, route_8,
    "Parasol Lady Lumi")
level_up(battle,
    mienfoo,
    33)
battle = make_battle(playthrough, route_8,
    "Fisherman Bruce")
battle = make_battle(playthrough, route_8,
    "PKMN Ranger Annie")
level_up(battle,
    mienfoo,
    34)
moor_of_icirrus = make_location(name="Moor of Icirrus", region="Unova")
battle = make_battle(playthrough, moor_of_icirrus,
    "PKMN Ranger Harry")
battle = make_battle(playthrough, moor_of_icirrus,
    "PKMN Ranger Chloris")
level_up(battle,
    mienfoo,
    35)
battle = make_battle(playthrough, moor_of_icirrus,
    "Fisherman Damon")
icirrus_city = make_location(name="Icirrus City", region="Unova")
battle = make_battle(playthrough, icirrus_city,
    "Black Belt Grant")
level_up(battle,
    mienfoo,
    36)
battle = make_battle(playthrough, icirrus_city,
    "Battle Girl Miriam")
level_up(battle,
    pansear,
    42)
battle = make_battle(playthrough, icirrus_city,
    "Black Belt Kendrew")
battle = make_battle(playthrough, icirrus_city,
    "Battle Girl Mikiko")
battle = make_battle(playthrough, icirrus_city,
    "Battle Girl Chandra")
level_up(battle,
    mienfoo,
    37)
battle = make_battle(playthrough, icirrus_city,
    "Black Belt Thomas")
battle = make_battle(playthrough, icirrus_city,
    "Leader Brycen")
level_up(battle,
    stoutland,
    43)
level_up(battle,
    mienfoo,
    38)
battle = make_battle(playthrough, dragonspiral_tower,
    "Team Plasma Grunt")
battle = make_battle(playthrough, dragonspiral_tower,
    "Team Plasma Grunt")
battle = make_battle(playthrough, dragonspiral_tower,
    "Team Plasma Grunt")
battle = make_battle(playthrough, dragonspiral_tower,
    "Team Plasma Grunt")
level_up(battle,
    archeops,
    42)
battle = make_battle(playthrough, dragonspiral_tower,
    "Team Plasma Grunt")
level_up(battle,
    pansear,
    43)
level_up(battle,
    mienfoo,
    39)
simisear = evolve(battle,
    pansear,
    "Simisear",
    dex_no=514,
    type1="Fire")
battle = make_battle(playthrough, dragonspiral_tower,
    "Team Plasma Grunt")
battle = make_battle(playthrough, dragonspiral_tower,
    "Team Plasma Grunt")
battle = make_battle(playthrough, dragonspiral_tower,
    "Team Plasma Grunt")
battle = make_battle(playthrough, dragonspiral_tower,
    "Team Plasma Grunt")
battle = make_battle(playthrough, relic_castle,
    "Team Plasma Grunt")
level_up(battle,
    mienfoo,
    40)
level_up(battle,
    seismitoad,
    43)
battle = make_battle(playthrough, relic_castle,
    "Team Plasma Grunt")
battle = make_battle(playthrough, relic_castle,
    "Team Plasma Grunt")
battle = make_battle(playthrough, relic_castle,
    "Team Plasma Grunt")
battle = make_battle(playthrough, relic_castle,
    "Team Plasma Grunt")
battle = make_battle(playthrough, relic_castle,
    "Team Plasma Grunt")
level_up(battle,
    mienfoo,
    41)
battle = make_battle(playthrough, relic_castle,
    "Team Plasma Grunt")
level_up(battle,
    archeops,
    43)
battle = make_battle(playthrough, route_8,
    "PKMN Trainer Bianca")
level_up(battle,
    mienfoo,
    42)
route_9 = make_location(name="Route 9", region="Unova")
battle = make_battle(playthrough, route_9,
    "Biker Phillip")
battle = make_battle(playthrough, route_9,
    "Roughneck Reese")
battle = make_battle(playthrough, route_9,
    "Hooligans Jim & Cas",
    battle_type="Double")
battle = make_battle(playthrough, route_9,
    "Biker Zeke")
battle = make_battle(playthrough, route_9,
    "Roughneck Chance")
battle = make_battle(playthrough, route_9,
    "Waitress Flo")
level_up(battle,
    mienfoo,
    43)
battle = make_battle(playthrough, route_9,
    "Rich Boy Manuel")
level_up(battle,
    archeops,
    44)
battle = make_battle(playthrough, route_9,
    "Lady Isabel")
battle = make_battle(playthrough, route_9,
    "Waiter Bert")
battle = make_battle(playthrough, route_9,
    "Wild Pawniard")
pawniard = catch(battle,
    slot=6,
    species="Pawniard",
    dex_no=624,
    type1="Dark",
    type2="Steel",
    caught_date=dt.date(2023, 8, 26),
    caught_level=32,
    ball="Ultra Ball",
    gender='M')
opelucid_city = make_location(name="Opelucid City", region="Unova")
battle = make_battle(playthrough, opelucid_city,
    "Ace Trainer Eileen",
    battle_type="Rotation")
level_up(battle,
    pawniard,
    33)
battle = make_battle(playthrough, opelucid_city,
    "Ace Trainer Lou",
    battle_type="Rotation")
battle = make_battle(playthrough, opelucid_city,
    "Ace Trainer Webster")
battle = make_battle(playthrough, opelucid_city,
    "Ace Trainer Olwen")
level_up(battle,
    pawniard,
    34)
battle = make_battle(playthrough, opelucid_city,
    "Ace Trainer Jose")
level_up(battle,
    stoutland,
    44)
battle = make_battle(playthrough, opelucid_city,
    "Ace Trainer Clara")
battle = make_battle(playthrough, opelucid_city,
    "Veteran Hugo")
level_up(battle,
    pawniard,
    35)
battle = make_battle(playthrough, opelucid_city,
    "Ace Trainer Tom")
battle = make_battle(playthrough, opelucid_city,
    "Ace Trainer Dara")
level_up(battle,
    pawniard,
    36)
battle = make_battle(playthrough, opelucid_city,
    "Veteran Kim")
battle = make_battle(playthrough, opelucid_city,
    "Leader Drayden")
level_up(battle,
    pawniard,
    37)
level_up(battle,
    seismitoad,
    44)
route_10 = make_location(name="Route 10", region="Unova")
battle = make_battle(playthrough, route_10,
    "Battle Girl Amy")
level_up(battle,
    pawniard,
    38)
battle = make_battle(playthrough, route_10,
    "Ace Trainer Johan")
level_up(battle,
    archeops,
    45)
battle = make_battle(playthrough, route_10,
    "Veteran Karla")
level_up(battle,
    pawniard,
    39)
battle = make_battle(playthrough, route_10,
    "PKMN Trainer Cheren")
level_up(battle,
    pawniard,
    40)
level_up(battle,
    simisear,
    44)
battle = make_battle(playthrough, route_10,
    "Black Belt Corey")
battle = make_battle(playthrough, route_10,
    "Hiker Bret")
level_up(battle,
    pawniard,
    41)
battle = make_battle(playthrough, route_10,
    "Ace Trainer Cheyenne")
level_up(battle,
    mienfoo,
    44)
level_up(battle,
    stoutland,
    45)
battle = make_battle(playthrough, route_10,
    "Veteran Chester")
level_up(battle,
    pawniard,
    42)
victory_road = make_location(name="Victory Road", region="Unova")
battle = make_battle(playthrough, victory_road,
    "Ace Trainer Shanta")
battle = make_battle(playthrough, victory_road,
    "Ace Trainer Dwayne")
level_up(battle,
    pawniard,
    43)
battle = make_battle(playthrough, victory_road,
    "Veteran Tiffany")
battle = make_battle(playthrough, victory_road,
    "Ace Trainer Cathy")
level_up(battle,
    archeops,
    46)
level_up(battle,
    pawniard,
    44)
battle = make_battle(playthrough, victory_road,
    "Black Belt Tyrone")
battle = make_battle(playthrough, victory_road,
    "Doctor Logan")
level_up(battle,
    archeops,
    47)
level_up(battle,
    pawniard,
    45)
battle = make_battle(playthrough, victory_road,
    "Ace Trainer David")
level_up(battle,
    seismitoad,
    45)
battle = make_battle(playthrough, victory_road,
    "Veteran Martell",
    lost=True)
battle = make_battle(playthrough, victory_road,
    "Veteran Martell")
level_up(battle,
    seismitoad,
    46)
pokemon_league = make_location(name="Pokémon League", region="Unova")
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal")
level_up(battle,
    mienfoo,
    45)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Shauntal",
    lost=True)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Grimsley",
    lost=True)
level_up(battle,
    simisear,
    45)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Caitlin")
level_up(battle,
    stoutland,
    46)
level_up(battle,
    archeops,
    48)
level_up(battle,
    mienfoo,
    46)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Grimsley",
    lost=True)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal",
    lost=True)
level_up(battle,
    pawniard,
    46)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal",
    lost=True)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal")
level_up(battle,
    simisear,
    46)
level_up(battle,
    simisear,
    47)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Caitlin",
    lost=True)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal",
    lost=True)
level_up(battle,
    pawniard,
    47)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal")
level_up(battle,
    mienfoo,
    47)
level_up(battle,
    archeops,
    49)
level_up(battle,
    seismitoad,
    47)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Caitlin",
    lost=True)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal")
level_up(battle,
    stoutland,
    47)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Caitlin")
level_up(battle,
    stoutland,
    48)
level_up(battle,
    archeops,
    50)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Grimsley",
    lost=True)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal",
    lost=True)
level_up(battle,
    seismitoad,
    48)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal")
level_up(battle,
    mienfoo,
    48)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Caitlin",
    lost=True)
level_up(battle,
    stoutland,
    49)
level_up(battle,
    mienfoo,
    49)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal",
    lost=True)
level_up(battle,
    simisear,
    48)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal")
level_up(battle,
    archeops,
    51)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Caitlin")
level_up(battle,
    pawniard,
    48)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Shauntal",
    lost=True)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal",
    lost=True)
level_up(battle,
    pawniard,
    49)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal")
level_up(battle,
    archeops,
    52)
level_up(battle,
    simisear,
    49)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Caitlin")
level_up(battle,
    simisear,
    50)
level_up(battle,
    seismitoad,
    49)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Grimsley",
    lost=True)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal")
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Caitlin")
level_up(battle,
    mienfoo,
    50)
mienshao = evolve(battle,
    mienfoo,
    "Mienshao",
    dex_no=620,
    type1="Fighting")
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Grimsley")
level_up(battle,
    mienshao,
    51)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Shauntal",
    lost=True)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Grimsley")
level_up(battle,
    pawniard,
    50)
level_up(battle,
    mienshao,
    52)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal",
    lost=True)
level_up(battle,
    pawniard,
    51)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Grimsley")
level_up(battle,
    seismitoad,
    50)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal",)
level_up(battle,
    archeops,
    53)
level_up(battle,
    seismitoad,
    51)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Shauntal",
    lost=True)
level_up(battle,
    stoutland,
    50)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Grimsley")
level_up(battle,
    mienshao,
    53)
level_up(battle,
    simisear,
    51)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Marshal",)
level_up(battle,
    simisear,
    52)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Shauntal",)
battle = make_battle(playthrough, pokemon_league,
    "Elite Four Caitlin")
level_up(battle,
    simisear,
    53)
level_up(battle,
    simisear,
    54)
ns_castle = make_location(name="N's Castle", region="Unova")
battle = make_battle(playthrough, ns_castle,
    "Team Plasma N",
    lost=True)
level_up(battle,
    stoutland,
    51)
battle = make_battle(playthrough, ns_castle,
    "Team Plasma N",
    lost=True)
level_up(battle,
    stoutland,
    52)
level_up(battle,
    mienshao,
    54)
battle = make_battle(playthrough, ns_castle,
    "Team Plasma N",
    lost=True)
level_up(battle,
    stoutland,
    53)
battle = make_battle(playthrough, ns_castle,
    "Team Plasma N",)
level_up(battle,
    mienshao,
    55)
level_up(battle,
    simisear,
    55)
level_up(battle,
    stoutland,
    54)
level_up(battle,
    archeops,
    54)
battle = make_battle(playthrough, ns_castle,
    "Team Plasma Ghetsis")
level_up(battle,
    pawniard,
    52)
level_up(battle,
    seismitoad,
    52)
bisharp = evolve(battle,
    pawniard,
    "Bisharp",
    dex_no=625,
    type1="Dark",
    type2="Steel")
         


