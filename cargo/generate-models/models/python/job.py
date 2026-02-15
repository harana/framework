# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Job(BaseModel):
    """
    job
    
    ID fields: id
    """

    action_config: str
    action_type: str
    completed_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    duration_ms: Optional[int] = None
    error: Optional[str] = None
    error_details: str
    lock_expires_at: Optional[datetime] = None
    lock_token: Optional[str] = None
    max_retries: int = Field(default=3)
    metadata: str
    result: Optional[str] = None
    retry_attempt: int = Field(default=0)
    retry_at: Optional[datetime] = None
    schedule_id: str = Field(description="Reference: schedule.id")
    scheduled_at: datetime
    started_at: Optional[datetime] = None
    status: str = Field(default="pending")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    worker_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class JobStatus(BaseModel):
    """
    job_status
    
    ID fields: id
    """

    value: str = Field(default="pending")
    class Config:
        from_attributes = True
        populate_by_name = True


class JobQuery(BaseModel):
    """
    job_query
    
    ID fields: id
    """

    limit: Optional[int] = None
    offset: Optional[int] = None
    schedule_id: Optional[str] = None
    scheduled_after: Optional[datetime] = None
    scheduled_before: Optional[datetime] = None
    status: str
    class Config:
        from_attributes = True
        populate_by_name = True


class JobInfo(BaseModel):
    """
    job_info
    
    ID fields: id
    """

    action_type: str
    completed_at: Optional[datetime] = None
    duration_ms: Optional[int] = None
    error: Optional[str] = None
    job_id: str
    retry_attempt: int
    schedule_id: str
    schedule_name: str
    scheduled_at: datetime
    started_at: Optional[datetime] = None
    status: str
    worker_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


