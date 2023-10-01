from operator import and_
from sqlalchemy import select

from .models import *
from .pkmn_constants import *

def make_playthrough(
    id_no: str,
    name: str,
    version: str,
    adventure_started: dt.date
) -> tuple:
    """Create a new playthrough."""
    print(f"""
    let playthrough = create_playthrough(conn, "{id_no}", "{name}", "{version}", "{adventure_started.strftime('%Y-%m-%d')}");
    """)
    return "playthrough"


def make_location(
    name: str,
    region: str
) -> tuple:
    var = f"{name} {region}".lower().replace(" ", "_").replace("'","")
    """Create a new location."""
    print(f"""
    let {var} = create_location(conn, "{name}", "{region}");
    """)
    return var

trainer_class_dict = dict()
trainer_dict = dict()
def make_battle(
    playthrough: tuple,
    location: tuple,
    opponent: str,
    *,
    lost: bool = False,
    battle_type: str = "Single",
) -> tuple:
    """Create a new battle."""
    
    # get opponents
    opponents = opponent.split(" with ")[0]
    opponent1 = opponents.split(" and ")[0].strip()
    opponent2 = opponents.split(" and ")[1].strip() if " and " in opponents else None
    partner = opponent.split(" with ")[1].strip() if " with " in opponent else None

    # clean up trainer vars
    if opponent1 not in trainer_dict:
        trainer_dict[opponent1] = opponent1.lower().replace(" ", "_").replace("'","").replace("&","and")
        opponent1_class, *opponent1_name  = opponent1.rsplit(" ", 1) if "&" not in opponent1 else opponent1.split(" ", 3)
        opponent1_name = " ".join(opponent1_name)
        if opponent1_class not in trainer_class_dict:
            trainer_class_dict[opponent1_class] = opponent1_class.lower().replace(" ", "_").replace("'","")
            print(f"""
            let {trainer_class_dict[opponent1_class]} = create_trainer_class(conn, "{opponent1_class}");
            """)
        print(f"""
        let {trainer_dict[opponent1]} = create_trainer(conn, "{opponent1_name}", &{trainer_class_dict[opponent1_class]});
        """)
    if opponent2 and opponent2 not in trainer_dict:
        trainer_dict[opponent2] = opponent2.lower().replace(" ", "_").replace("'","").replace("&","and")
        opponent2_class, *opponent2_name  = opponent2.rsplit(" ", 1) if "&" not in opponent2 else opponent2.split(" ", 3)
        opponent2_name = " ".join(opponent2_name)
        if opponent2_class not in trainer_class_dict:
            trainer_class_dict[opponent2_class] = opponent2_class.lower().replace(" ", "_").replace("'","")
            print(f"""
            let {trainer_class_dict[opponent2_class]} = create_trainer_class(conn, "{opponent2_class}");
            """)
        print(f"""
        let {trainer_dict[opponent2]} = create_trainer(conn, "{opponent2_name}", &{trainer_class_dict[opponent2_class]});
        """)
    if partner and partner not in trainer_dict:
        trainer_dict[partner] = partner.lower().replace(" ", "_").replace("'","").replace("&","and")
        partner_class, *partner_name  = partner.rsplit(" ", 1) if "&" not in partner else partner.split(" ", 3)
        partner_name = " ".join(partner_name)
        if partner_class not in trainer_class_dict:
            trainer_class_dict[partner_class] = partner_class.lower().replace(" ", "_").replace("'","")
            print(f"""
            let {trainer_class_dict[partner_class]} = create_trainer_class(conn, "{partner_class}");
            """)
        print(f"""
        let {trainer_dict[partner]} = create_trainer(conn, "{partner_name}", &{trainer_class_dict[partner_class]});
        """)


    print(f"""
    let battle = create_battle(
        conn,
        &{playthrough},
        &{location},
        &{trainer_dict[opponent1]},
        {f"Some(&{trainer_dict[opponent2]})" if opponent2 else "None"},
        {f"Some(&{trainer_dict[partner]})" if partner else "None"},
        "{battle_type}",
        &0,
        &{"true" if lost else "false"},
    );
    """)
    return "battle"



def receive_pokemon(
    playthrough: tuple,
    slot: int,
    species: str,
    *,
    caught_date: dt.date,
    caught_location: tuple,
    caught_level: int,
    ball: str,
    dex_no: int,
    type1: str,
    type2: str = None,
    gender: str = None,
    nickname: str = None,
) -> tuple:
    """Receive a new pokemon as a gift or prize."""
    species_var = "species_" + species.lower().replace(" ", "_").replace("'","")
    print(f"""
        let {species_var} = create_species(conn, &{dex_no}, "{species}", None, &5, "{type1}", {f'Some("{type2}")' if type2 else "None"});
    """)

    team_member_var = "team_member_" + species.lower().replace(" ", "_").replace("'","")
    print(f"""
        let {team_member_var} = catch_pokemon(
            conn,
            &{playthrough},
            &{slot},
            &{species_var},
            {f'Some("{nickname}")' if nickname else "None"},
            "Gift",
            "{caught_date.strftime('%Y-%m-%d')}",
            &{caught_location},
            &{caught_level},
            "{gender if gender else "N"}",
            "{ball}",
        );
    """)
    return team_member_var
    

def revive_fossil(
    playthrough: tuple,
    fossil: str,
    slot: int,
    species: str,
    *,
    caught_date: dt.date,
    caught_location: tuple,
    caught_level: int,
    ball: str,
    dex_no: int,
    type1: str,
    type2: str = None,
    gender: str = None,
    nickname: str = None,
) -> tuple:
    """Revive a Fossil"""
    species_var = "species_" + species.lower().replace(" ", "_").replace("'","")
    print(f"""
        let {species_var} = create_species(conn, &{dex_no}, "{species}", None, &5, "{type1}", {f'Some("{type2}")' if type2 else "None"});
    """)

    team_member_var = "team_member_" + species.lower().replace(" ", "_").replace("'","")
    print(f"""
        let {team_member_var} = catch_pokemon(
            conn,
            &{playthrough},
            &{slot},
            &{species_var},
            {f'Some("{nickname}")' if nickname else "None"},
            "Fossil",
            "{caught_date.strftime('%Y-%m-%d')}",
            &{caught_location},
            &{caught_level},
            "{gender if gender else "N"}",
            "{ball}",
        );
    """)
    return team_member_var
    


def level_up(
    event: tuple,
    team_member: tuple,
    level: int,
) -> tuple:
    """Level up a pokemon."""
    # print(f"""
    # level_up(
    #     conn,
    #     &{event},
    #     &{team_member},
    #     &{level},
    # );
    # """)
    # return event
    

def evolve(
    event: tuple,
    team_member: tuple,
    species: str,
    dex_no: int,
    type1: str,
    type2: str = None,
) -> tuple:
    """Evolve a pokemon."""
    # with Session(engine) as session:
    #     event: Event = session.get(Event, event)
    #     team_member: TeamMember = session.get(TeamMember, team_member)
    #     old_team_member_name = team_member.to_str(session)
    #     team_member_entry = TeamMemberEntry(
    #         team_member = team_member,
    #         event = event,
    #         species = species,
    #         dex_no = dex_no,
    #         type1 = type1,
    #         type2 = type2,
    #     )
    #     team_member_entry = session.merge(team_member_entry)
    #     session.commit()
    #     print(f"{old_team_member_name} evolved into {species}")
    #     return team_member.pk
    

def catch(
    battle: tuple,
    slot: int,
    species: str,
    *,
    dex_no: int,
    type1: str,
    caught_date: dt.date,
    caught_level: int,
    ball: str,
    type2: str = None,
    gender: str = None,
    nickname: str = None,
) -> tuple:
    """Catching a pokemon"""
    # with Session(engine) as session:
    #     battle: Event = session.get(Event, battle)
    #     team_member = TeamMember(
    #         playthrough=battle.playthrough,
    #         slot=slot,
    #         nickname=nickname,
    #         caught_date=caught_date,
    #         caught_location=battle.location,
    #         caught_level=caught_level,
    #         ball=ball,
    #         gender=gender,
    #     )
    #     team_member = session.merge(team_member)
    #     team_member_entry = TeamMemberEntry(
    #         team_member=team_member,
    #         event=battle,
    #         level=caught_level,
    #         species=species,
    #         dex_no=dex_no,
    #         type1=type1,
    #         type2=type2,
    #     )
    #     session.merge(team_member_entry)
    #     session.commit()
    #     print(f"{team_member.to_str(session)} was caught")
    #     return team_member.pk



        

