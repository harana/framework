# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Job(BaseModel):
    """
    job
    
    ID fields: name, queue, created_at
    """

    completed_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    error_message: Optional[str] = None
    max_retries: int = Field(default=3)
    payload: Optional[str] = None
    priority: int = Field(default=0)
    queue: str = Field(default="default")
    result: Optional[str] = None
    retry_count: int = Field(default=0)
    scheduled_at: Optional[datetime] = None
    started_at: Optional[datetime] = None
    status: str = Field(default="pending")
    timeout_seconds: int = Field(default=300)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class JobSchedule(BaseModel):
    """
    job_schedule
    
    ID fields: id
    """

    cron_expression: str
    is_active: bool = Field(default=True)
    job_name: str
    last_run_at: Optional[datetime] = None
    next_run_at: Optional[datetime] = None
    payload: Optional[str] = None
    timezone: str = Field(default="utc")
    class Config:
        from_attributes = True
        populate_by_name = True


class JobLog(BaseModel):
    """
    job_log
    
    ID fields: job_id, created_at
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    job_id: str = Field(description="Reference: job.id")
    level: str = Field(default="info")
    message: str
    class Config:
        from_attributes = True
        populate_by_name = True


