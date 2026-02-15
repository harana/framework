# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsEventbridgeEventBus(BaseModel):
    """
    aws_eventbridge_event_bus
    
    ID fields: account_id, name
    """

    account_id: str
    arn: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    name: str
    policy: Optional[str] = None
    region: Optional[str] = None
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEventbridgeRule(BaseModel):
    """
    aws_eventbridge_rule
    
    ID fields: event_bus_id, name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    event_bus_id: str = Field(description="Reference: aws_eventbridge_event_bus.id")
    event_pattern: Optional[str] = None
    name: str
    role_arn: Optional[str] = None
    rule_arn: Optional[str] = None
    schedule_expression: Optional[str] = None
    state: str = Field(default="enabled")
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEventbridgeTarget(BaseModel):
    """
    aws_eventbridge_target
    
    ID fields: rule_id, target_id
    """

    arn: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    input: Optional[str] = None
    input_path: Optional[str] = None
    input_transformer: Optional[str] = None
    role_arn: Optional[str] = None
    rule_id: str = Field(description="Reference: aws_eventbridge_rule.id")
    target_id: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEventbridgeArchive(BaseModel):
    """
    aws_eventbridge_archive
    
    ID fields: account_id, archive_name
    """

    account_id: str
    archive_arn: Optional[str] = None
    archive_name: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    event_count: int = Field(default=0)
    event_pattern: Optional[str] = None
    event_source_arn: str
    region: Optional[str] = None
    retention_days: Optional[int] = None
    size_bytes: int = Field(default=0)
    state: str = Field(default="creating")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEventbridgeReplay(BaseModel):
    """
    aws_eventbridge_replay
    
    ID fields: account_id, replay_name
    """

    account_id: str
    description: Optional[str] = None
    event_end_time: datetime
    event_source_arn: str
    event_start_time: datetime
    region: Optional[str] = None
    replay_arn: Optional[str] = None
    replay_end_time: Optional[datetime] = None
    replay_name: str
    replay_start_time: Optional[datetime] = None
    state: str = Field(default="starting")
    state_reason: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


