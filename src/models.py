from typing import List
from typing import Optional
import datetime as dt

import sqlalchemy
from sqlalchemy import ForeignKeyConstraint, create_engine, Column, Integer, String, Date
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column, relationship, Session
import pandas as pd

from .constants import *


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

    def __str__(self):
        return f"{self.version} {self.adventure_started}"
    
    @property
    def pk(self):
        return (self.id_no,)

class Region(Base):
    """A region in the game."""
    __tablename__ = "region"
    name: Mapped[str] = mapped_column(String(64), primary_key=True)

    def __str__(self):
        return f"{self.name}"
    
    @property
    def pk(self):
        return (self.name,)

class Location(Base):
    """A location in a region."""
    __tablename__ = "location"
    name: Mapped[str] = mapped_column(String(64), primary_key=True)
    region_name: Mapped[str] = mapped_column(String(64), primary_key=True)

    # relationships
    region: Mapped[Region] = relationship(Region, foreign_keys=[region_name])

    # table args
    __table_args__ = (
        ForeignKeyConstraint([region_name], [Region.name]),
    )

    def __str__(self):
        return f"{self.name} ({self.region})"
    
    @property
    def pk(self):
        return (self.name, *self.region.pk)
    

class Type(Base):
    """A type of Pokemon."""
    __tablename__ = "type"
    name: Mapped[str] = mapped_column(String(32), primary_key=True)

    def __str__(self):
        return f"{self.name}"
    
    @property
    def pk(self):
        return (self.name,)
    

class Species(Base):
    """
    A species of pokemon.
    Can also represent several forms of the same species.
    """
    __tablename__ = "species"
    name: Mapped[str] = mapped_column(String(64), primary_key=True)
    dex_no: Mapped[int] = mapped_column(Integer)
    type1_name: Mapped[str] = mapped_column(String(32))
    type2_name: Mapped[Optional[str]] = mapped_column(String(32), nullable=True)

    # relationships
    type1: Mapped[str] = relationship(Type, foreign_keys=[type1_name])
    type2: Mapped[Optional[str]] = relationship(Type, foreign_keys=[type2_name])

    # foreign key constraints
    __table_args__ = (
        ForeignKeyConstraint([type1_name], [Type.name]),
        ForeignKeyConstraint([type2_name], [Type.name]),
    )

    def __str__(self):
        return f"{self.name}"
    
    @property
    def pk(self):
        return (self.name,)
    

class EventType(Base):
    """An event type."""
    __tablename__ = "event_type"
    name: Mapped[str] = mapped_column(String(32), primary_key=True)

    def __str__(self):
        return f"{self.name}"
    
    @property
    def pk(self):
        return (self.name,)
    
class Event(Base):
    """An event in the game."""
    __tablename__ = "event"
    no: Mapped[int] = mapped_column(Integer, primary_key=True)
    playthrough_id_no: Mapped[str] = mapped_column(String(5))
    location_name: Mapped[str] = mapped_column(String(64))
    location_region: Mapped[str] = mapped_column(String(64))
    event_type_name: Mapped[str] = mapped_column(String(32))
    event_name: Mapped[Optional[str]] = mapped_column(String(128), nullable=True)
    round: Mapped[Optional[int]] = mapped_column(Integer, nullable=True, default=0)

    #relationships
    playthrough: Mapped[str] = relationship(Playthrough, foreign_keys=[playthrough_id_no])
    location: Mapped[Location] = relationship(Location, foreign_keys=[location_name, location_region])
    event_type: Mapped[str] = relationship(EventType, foreign_keys=[event_type_name])
    team_member_entries: Mapped[List["TeamMemberEntry"]] = relationship("TeamMemberEntry", back_populates="event")

    # foreign key constraints
    __table_args__ = (
        ForeignKeyConstraint([playthrough_id_no], [Playthrough.id_no]),
        ForeignKeyConstraint([location_name, location_region], [Location.name, Location.region_name]),
        ForeignKeyConstraint([event_type_name], [EventType.name]),
    )

    def __str__(self):
        return f"{self.event_type} {self.event_name}"
    
    @property
    def pk(self):
        return (self.no,)
    

class Ball(Base):
    """A type of ball."""
    __tablename__ = "ball"
    name: Mapped[str] = mapped_column(String(32), primary_key=True)

    def __str__(self):
        return f"{self.name}"
    
    @property
    def pk(self):
        return (self.name,)


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
    ball_name: Mapped[str] = mapped_column(String(32))
    gender: Mapped[str] = mapped_column(String(1))

    # relationships
    playthrough: Mapped[str] = relationship(Playthrough, foreign_keys=[playthrough_id_no])
    caught_location: Mapped[Location] = relationship(Location, foreign_keys=[caught_location_name, caught_location_region])
    ball: Mapped[str] = relationship(Ball, foreign_keys=[ball_name])
    # TODO: this relationship is not working
    team_member_entries: Mapped[List['TeamMemberEntry']] = relationship(back_populates="team_member")

    # foreign key constraints
    __table_args__ = (
        ForeignKeyConstraint([playthrough_id_no], [Playthrough.id_no]),
        ForeignKeyConstraint([caught_location_name, caught_location_region], [Location.name, Location.region_name]),
        ForeignKeyConstraint([ball_name], [Ball.name]),
    )
    
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
        session.refresh(species)
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
    species_name: Mapped[str] = mapped_column(String(64), nullable=True)

    # relationships
    team_member: Mapped[TeamMember] = relationship(TeamMember, foreign_keys=[team_member_playthrough_id_no, team_member_slot])
    event: Mapped[Event] = relationship(Event, foreign_keys=[event_no])
    species: Mapped[Species] = relationship(Species, foreign_keys=[species_name])

    # foreign key constraints
    __table_args__ = (
        ForeignKeyConstraint([team_member_playthrough_id_no, team_member_slot], [TeamMember.playthrough_id_no, TeamMember.slot]),
        ForeignKeyConstraint([event_no], [Event.no]),
        ForeignKeyConstraint([species_name], [Species.name]),
    )

    def __str__(self):
        return f"{self.team_member} {self.species} {self.level}"
    
    @property
    def pk(self):
        return (self.team_member_playthrough_id_no, self.team_member_slot, self.event_no)
    
