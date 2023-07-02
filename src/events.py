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
    with Session(engine) as session:
        playthrough = Playthrough(
            id_no = id_no,
            name = name,
            version = version,
            adventure_started = adventure_started
        )
        session.merge(playthrough)
        session.commit()
        print(f"Created playthrough {playthrough}")
        return playthrough.pk


def make_location(
    name: str,
    region: str
) -> tuple:
    """Create a new location."""
    with Session(engine) as session:
        location = Location(
            name = name,
            region = region
        )
        location = session.merge(location)
        session.commit()
        print(f"Created location {location}")
        return location.pk



def make_battle(
    playthrough: tuple,
    location: tuple,
    opponent: str,
    *,
    lost: bool = False,
) -> tuple:
    """Create a new battle."""
    with Session(engine) as session:
        playthrough: Playthrough = session.get(Playthrough, playthrough)
        location: Location = session.get(Location, location)
        battle = Event(
            playthrough = playthrough,
            location = location,
            event_type = "Battle",
            event_name = opponent,
            failed = lost,
        )
        battle = session.merge(battle)
        session.commit()
        print(f"Battled {opponent} at {location}")
        return battle.pk



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
    with Session(engine) as session:
        playthrough: Playthrough = session.get(Playthrough, playthrough)
        caught_location: Location = session.get(Location, caught_location)
        event = Event(
            playthrough = playthrough,
            location = caught_location,
            event_type = "Gift",
            event_name = species,
        )
        event = session.merge(event)
        team_member = TeamMember(
            playthrough = playthrough,
            slot = slot,
            nickname = nickname,
            caught_date = caught_date,
            caught_location = caught_location,
            caught_level = caught_level,
            ball = ball,
            gender = gender,
        )
        team_member = session.merge(team_member)
        team_member_entry = TeamMemberEntry(
            team_member = team_member,
            event = event,
            level = caught_level,
            species = species,
            dex_no = dex_no,
            type1 = type1,
            type2 = type2,
        )
        team_member_entry = session.merge(team_member_entry)
        session.commit()
        print(f"Received {team_member.to_str(session)} as a gift")
        return team_member.pk
    

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
    with Session(engine) as session:
        playthrough: Playthrough = session.get(Playthrough, playthrough)
        caught_location: Location = session.get(Location, caught_location)
        event = Event(
            playthrough = playthrough,
            location = caught_location,
            event_type = "Revive",
            event_name = fossil,
        )
        event = session.merge(event)
        team_member = TeamMember(
            playthrough = playthrough,
            slot = slot,
            nickname = nickname,
            caught_date = caught_date,
            caught_location = caught_location,
            caught_level = caught_level,
            ball = ball,
            gender = gender,
        )
        team_member = session.merge(team_member)
        team_member_entry = TeamMemberEntry(
            team_member = team_member,
            event = event,
            level = caught_level,
            species = species,
            dex_no = dex_no,
            type1 = type1,
            type2 = type2,
        )
        team_member_entry = session.merge(team_member_entry)
        session.commit()
        print(f"Revived {team_member.to_str(session)} from a {fossil}")
        return team_member.pk
    


def level_up(
    event: tuple,
    team_member: tuple,
    level: int,
) -> tuple:
    """Level up a pokemon."""
    with Session(engine) as session:
        event: Event = session.get(Event, event)
        team_member: TeamMember = session.get(TeamMember, team_member)
        team_member_entry = TeamMemberEntry(
            team_member = team_member,
            event = event,
            level = level,
        )
        team_member_entry = session.merge(team_member_entry)
        session.commit()
        print(f"{team_member.to_str(session)} leveled up to {level}")
        return event.pk
    

def evolve(
    event: tuple,
    team_member: tuple,
    species: str,
    dex_no: int,
    type1: str,
    type2: str = None,
) -> tuple:
    """Evolve a pokemon."""
    with Session(engine) as session:
        event: Event = session.get(Event, event)
        team_member: TeamMember = session.get(TeamMember, team_member)
        old_team_member_name = team_member.to_str(session)
        team_member_entry = TeamMemberEntry(
            team_member = team_member,
            event = event,
            species = species,
            dex_no = dex_no,
            type1 = type1,
            type2 = type2,
        )
        team_member_entry = session.merge(team_member_entry)
        session.commit()
        print(f"{old_team_member_name} evolved into {species}")
        return team_member.pk
    

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
    with Session(engine) as session:
        battle: Event = session.get(Event, battle)
        team_member = TeamMember(
            playthrough=battle.playthrough,
            slot=slot,
            nickname=nickname,
            caught_date=caught_date,
            caught_location=battle.location,
            caught_level=caught_level,
            ball=ball,
            gender=gender,
        )
        team_member = session.merge(team_member)
        team_member_entry = TeamMemberEntry(
            team_member=team_member,
            event=battle,
            level=caught_level,
            species=species,
            dex_no=dex_no,
            type1=type1,
            type2=type2,
        )
        session.merge(team_member_entry)
        session.commit()
        print(f"{team_member.to_str(session)} was caught")
        return team_member.pk



        

