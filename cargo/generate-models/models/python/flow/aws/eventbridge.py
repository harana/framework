# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class PutEvents(BaseModel):
    """
    put_events
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateRule(BaseModel):
    """
    create_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteRule(BaseModel):
    """
    delete_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeRule(BaseModel):
    """
    describe_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListRules(BaseModel):
    """
    list_rules
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EnableRule(BaseModel):
    """
    enable_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DisableRule(BaseModel):
    """
    disable_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutTargets(BaseModel):
    """
    put_targets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveTargets(BaseModel):
    """
    remove_targets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTargetsByRule(BaseModel):
    """
    list_targets_by_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateEventBus(BaseModel):
    """
    create_event_bus
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteEventBus(BaseModel):
    """
    delete_event_bus
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeEventBus(BaseModel):
    """
    describe_event_bus
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListEventBuses(BaseModel):
    """
    list_event_buses
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutPermission(BaseModel):
    """
    put_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemovePermission(BaseModel):
    """
    remove_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateArchive(BaseModel):
    """
    create_archive
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteArchive(BaseModel):
    """
    delete_archive
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeArchive(BaseModel):
    """
    describe_archive
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StartReplay(BaseModel):
    """
    start_replay
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CancelReplay(BaseModel):
    """
    cancel_replay
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeReplay(BaseModel):
    """
    describe_replay
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TestEventPattern(BaseModel):
    """
    test_event_pattern
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


