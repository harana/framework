# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class EmitEvent(BaseModel):
    """
    emit_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SubscribeChannel(BaseModel):
    """
    subscribe_channel
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UnsubscribeChannel(BaseModel):
    """
    unsubscribe_channel
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BroadcastEvent(BaseModel):
    """
    broadcast_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetEvent(BaseModel):
    """
    get_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListEvents(BaseModel):
    """
    list_events
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AcknowledgeEvent(BaseModel):
    """
    acknowledge_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReplayEvents(BaseModel):
    """
    replay_events
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Event(BaseModel):
    """
    event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EventMetadata(BaseModel):
    """
    event_metadata
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EventRecord(BaseModel):
    """
    event_record
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


