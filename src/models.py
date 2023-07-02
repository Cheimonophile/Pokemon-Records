from typing import List
from typing import Optional
import datetime as dt

import sqlalchemy
from sqlalchemy import ForeignKeyConstraint, create_engine, Column, Integer, String, Date
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column, relationship, Session
import pandas as pd

from .constants import *
from .pkmn_constants import *


engine = create_engine(f'sqlite:///{DB_PATH}', echo=False)

class Base(DeclarativeBase):
    pass


class Playthrough(Base):
    """A playthrough of the game."""

    # fields
    __tablename__ = "playthrough"
    id_no: Mapped[str] = mapped_column(String(5), primary_key=True)
    name: Mapped[str] = mapped_column(String(12))
    version: Mapped[str] = mapped_column(String(64))
    adventure_started: Mapped[dt.date] = mapped_column(Date)

    def __init__(self, **kwargs):
        assert kwargs['version'] in VERSIONS, f"Invalid version {kwargs.get('version')}"
        super().__init__(**kwargs)

    def __str__(self):
        return f"{self.version} {self.adventure_started}"
    
    @property
    def pk(self):
        return (self.id_no,)

class Location(Base):
    """A location in a region."""
    __tablename__ = "location"
    name: Mapped[str] = mapped_column(String(64), primary_key=True)
    region: Mapped[str] = mapped_column(String(64), primary_key=True)

    def __init__(self, **kwargs):
        assert kwargs['region'] in REGIONS, f"Invalid region {kwargs.get('region')}"
        super().__init__(**kwargs)

    def __str__(self):
        return f"{self.name} ({self.region})"
    
    @property
    def pk(self):
        return (self.name, self.region)
    

    
class Event(Base):
    """An event in the game."""
    __tablename__ = "event"
    no: Mapped[int] = mapped_column(Integer, primary_key=True)
    playthrough_id_no: Mapped[str] = mapped_column(String(5))
    location_name: Mapped[str] = mapped_column(String(64))
    location_region: Mapped[str] = mapped_column(String(64))
    event_type: Mapped[str] = mapped_column(String(32))
    event_name: Mapped[Optional[str]] = mapped_column(String(128), nullable=True)
    round: Mapped[Optional[int]] = mapped_column(Integer, nullable=True, default=0)

    #relationships
    playthrough: Mapped[str] = relationship(Playthrough, foreign_keys=[playthrough_id_no])
    location: Mapped[Location] = relationship(Location, foreign_keys=[location_name, location_region])
    team_member_entries: Mapped[List["TeamMemberEntry"]] = relationship("TeamMemberEntry", back_populates="event")

    # foreign key constraints
    __table_args__ = (
        ForeignKeyConstraint([playthrough_id_no], [Playthrough.id_no]),
        ForeignKeyConstraint([location_name, location_region], [Location.name, Location.region]),
    )

    def __init__(self, **kwargs):
        assert kwargs['event_type'] in EVENT_TYPES, f"Invalid event type {kwargs.get('event_type')}"
        super().__init__(**kwargs)

    def __str__(self):
        return f"{self.event_type} {self.event_name}"
    
    @property
    def pk(self):
        return (self.no,)


class TeamMember(Base):
    """A team member."""
    __tablename__ = "team_member"

    playthrough_id_no: Mapped[str] = mapped_column(String(5), primary_key=True)
    slot: Mapped[int] = mapped_column(Integer, primary_key=True)
    nickname: Mapped[Optional[str]] = mapped_column(String(64), nullable=True)
    caught_date: Mapped[dt.date] = mapped_column(Date)
    caught_location_name: Mapped[str] = mapped_column(String(64))
    caught_location_region: Mapped[str] = mapped_column(String(64))
    caught_level: Mapped[int] = mapped_column(Integer)
    ball: Mapped[str] = mapped_column(String(32))
    gender: Mapped[str] = mapped_column(String(1), nullable=True)

    # relationships
    playthrough: Mapped[str] = relationship(Playthrough, foreign_keys=[playthrough_id_no])
    caught_location: Mapped[Location] = relationship(Location, foreign_keys=[caught_location_name, caught_location_region])
    team_member_entries: Mapped[List['TeamMemberEntry']] = relationship(back_populates="team_member")

    # foreign key constraints
    __table_args__ = (
        ForeignKeyConstraint([playthrough_id_no], [Playthrough.id_no]),
        ForeignKeyConstraint([caught_location_name, caught_location_region], [Location.name, Location.region]),
    )

    def __init__(self, **kwargs):
        assert kwargs['ball'] in BALLS, f"Invalid ball {kwargs['ball']}"
        assert kwargs.get('gender') in [None,'F','M'], f"Invalid gender {kwargs.get('gender')}"
        super().__init__(**kwargs)
    
    def to_str(self, session: Session):
        session.refresh(self)
        species = (
            session
            .query(TeamMemberEntry)
            .filter(TeamMemberEntry.team_member == self)
            .filter(TeamMemberEntry.species != None)
            .order_by(TeamMemberEntry.no.desc())
            .first()
            .species
        )
        return f"{self.nickname or species}"
    
    @property
    def pk(self):
        return (self.playthrough_id_no, self.slot)


class TeamMemberEntry(Base):
    """Entries for the current state of team members."""
    __tablename__ = "team_member_entry"
    no = mapped_column(Integer, primary_key=True)
    team_member_playthrough_id_no: Mapped[str] = mapped_column(String(5))
    team_member_slot: Mapped[int] = mapped_column(Integer)
    event_no: Mapped[int] = mapped_column(Integer)
    level: Mapped[int] = mapped_column(Integer, nullable=True)
    species: Mapped[str] = mapped_column(String(64), nullable=True)
    dex_no: Mapped[int] = mapped_column(Integer, nullable=True)
    type1: Mapped[str] = mapped_column(String(32), nullable=True)
    type2: Mapped[Optional[str]] = mapped_column(String(32), nullable=True)

    # relationships
    team_member: Mapped[TeamMember] = relationship(TeamMember, foreign_keys=[team_member_playthrough_id_no, team_member_slot])
    event: Mapped[Event] = relationship(Event, foreign_keys=[event_no])

    # foreign key constraints
    __table_args__ = (
        ForeignKeyConstraint([team_member_playthrough_id_no, team_member_slot], [TeamMember.playthrough_id_no, TeamMember.slot]),
        ForeignKeyConstraint([event_no], [Event.no]),
    )

    def __init__(self, **kwargs):
        assert kwargs.get('type1') in [None] + TYPES, f"Invalid type {kwargs['type1']}"
        assert kwargs.get('type2') in [None] + TYPES, f"Invalid type {kwargs['type2']}"
        super().__init__(**kwargs)

    def __str__(self):
        return f"{self.team_member} {self.species} {self.level}"
    
    @property
    def pk(self):
        return (self.no,)
    
