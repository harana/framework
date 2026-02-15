# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Schedule(BaseModel):
    """
    schedule
    
    ID fields: id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    cron_expression: Optional[str] = None
    description: Optional[str] = None
    enabled: bool = Field(default=True)
    interval_seconds: Optional[int] = None
    last_run_at: Optional[datetime] = None
    metadata: Optional[str] = None
    name: str
    next_run_at: Optional[datetime] = None
    schedule_type: str = Field(default="cron")
    timezone: str = Field(default="utc")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class ScheduleExecution(BaseModel):
    """
    schedule_execution
    
    ID fields: schedule_id, started_at
    """

    completed_at: Optional[datetime] = None
    duration_ms: Optional[int] = None
    error_message: Optional[str] = None
    output: Optional[str] = None
    schedule_id: str = Field(description="Reference: schedule.id")
    started_at: datetime = Field(default_factory=datetime.utcnow)
    status: str = Field(default="pending")
    triggered_by: str = Field(default="scheduled")
    class Config:
        from_attributes = True
        populate_by_name = True


