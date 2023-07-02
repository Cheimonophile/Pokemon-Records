from operator import and_
from sqlalchemy import select
from .models import *

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


def make_region(
    name: str
) -> tuple:
    """Create a new region."""
    with Session(engine) as session:
        region = Region(
            name = name
        )
        region = session.merge(region)
        session.commit()
        print(f"Created region {region}")
        return region.pk


def make_location(
    name: str,
    region: tuple
) -> tuple:
    """Create a new location."""
    with Session(engine) as session:
        region: Region = session.get(Region, region)
        location = Location(
            name = name,
            region = region
        )
        location = session.merge(location)
        session.commit()
        print(f"Created location {location}")
        return location.pk


def make_species(
    name: str,
    dex_no: int,
    type1: tuple,
    type2: Optional[tuple] = None
) -> tuple:
    """Create a new species."""
    with Session(engine) as session:
        species = Species(
            name = name,
            dex_no = dex_no,
            type1 = session.get(Type,type1),
            type2 = session.get(Type,type1)
        )
        species = session.merge(species)
        session.commit()
        print(f"Created species {species}")
        return species.pk


def make_type(
    name: str
) -> tuple:
    """Create a new type."""
    with Session(engine) as session:
        type = Type(
            name = name
        )
        type = session.merge(type)
        session.commit()
        print(f"Created type {type}")
        return type.pk

def make_event_type(
    name: str
) -> tuple:
    """Create a new event type."""
    with Session(engine) as session:
        event_type = EventType(
            name = name
        )
        event_type = session.merge(event_type)
        session.commit()
        print(f"Created event type {event_type}")
        return event_type.pk


def make_battle(
    playthrough: tuple,
    location: tuple,
    opponent: str,
) -> tuple:
    """Create a new battle."""
    with Session(engine) as session:
        playthrough: Playthrough = session.get(Playthrough, playthrough)
        location: Location = session.get(Location, location)
        event_type = session.get(EventType, "Battle")
        battle = Event(
            playthrough = playthrough,
            location = location,
            event_type = event_type,
            event_name = opponent,
        )
        battle = session.merge(battle)
        session.commit()
        print(f"Created battle {opponent} at {location}")
        return battle.pk


def make_ball(
    name: str
) -> tuple:
    """Create a new ball."""
    with Session(engine) as session:
        ball = Ball(
            name = name
        )
        ball = session.merge(ball)
        session.commit()
        print(f"Created ball {ball}")
        return ball.pk


def receive_pokemon(
    playthrough: tuple,
    species: tuple,
    slot: int,
    caught_date: dt.date,
    caught_location: tuple,
    caught_level: int,
    ball: tuple,
    gender: str = None,
    nickname: str = None,
) -> tuple:
    """Receive a new pokemon as a gift or prize."""
    assert gender in ["M", "F", None], f"{gender} is not a valid gender"
    with Session(engine) as session:
        playthrough: Playthrough = session.get(Playthrough, playthrough)
        caught_location: Location = session.get(Location, caught_location)
        species: Species = session.get(Species, species)
        ball: Ball = session.get(Ball, ball)
        event = Event(
            playthrough = playthrough,
            location = caught_location,
            event_type_name = "Gift",
            event_name = species.name,
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
        )
        team_member_entry = session.merge(team_member_entry)
        session.commit()
        print(f"Received {team_member.to_str(session)} as a gift")
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
        message = f"{team_member.to_str(session)} leveled up to {level}"
        print(message)
        return event.pk
    

def evolve(
        event: tuple,
        team_member: tuple,
        species: tuple,
) -> tuple:
    """Evolve a pokemon."""
    with Session(engine) as session:
        event: Event = session.get(Event, event)
        team_member: TeamMember = session.get(TeamMember, team_member)
        species: Species = session.get(Species, species)
        team_member_entry = TeamMemberEntry(
            team_member = team_member,
            event = event,
            species = species,
        )
        team_member_entry = session.merge(team_member_entry)
        session.commit()
        message = f"{team_member.to_str(session)} evolved into {species.name}"
        print(message)
        return team_member.pk
    

def catch(
    battle: tuple,
    species: tuple,
    slot: int,
    caught_date: dt.date,
    caught_level: int,
    ball: tuple,
    gender: str = None,
    nickname: str = None,
) -> tuple:
    """Catching a pokemon"""
    with Session(engine) as session:
        battle: Event = session.get(Event, battle)
        species: Species = session.get(Species, species)
        ball: Ball = session.get(Ball, ball)
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
        )
        session.merge(team_member_entry)
        session.commit()
        message = f"{team_member.to_str(session)} was caught"
        print(message)
        return team_member.pk



        

