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

    action_config: str
    action_type: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    cron_expression: Optional[str] = None
    description: Optional[str] = None
    end_at: Optional[datetime] = None
    execution_count: int = Field(default=0)
    interval_seconds: Optional[int] = None
    last_run_at: Optional[datetime] = None
    max_executions: Optional[int] = None
    metadata: str
    name: str
    next_run_at: Optional[datetime] = None
    owner_id: Optional[str] = None
    resume_at: Optional[datetime] = None
    retry_config: str
    run_at: Optional[datetime] = None
    schedule_type: str = Field(default="cron")
    start_at: Optional[datetime] = None
    status: str = Field(default="active")
    tags: List[str]
    timezone: str = Field(default="u_t_c")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    version: int = Field(default=1)
    class Config:
        from_attributes = True
        populate_by_name = True


class ScheduleStatus(BaseModel):
    """
    schedule_status
    
    ID fields: id
    """

    value: str = Field(default="active")
    class Config:
        from_attributes = True
        populate_by_name = True


class ScheduleType(BaseModel):
    """
    schedule_type
    
    ID fields: id
    """

    value: str = Field(default="cron")
    class Config:
        from_attributes = True
        populate_by_name = True


class ScheduleExecutionHistory(BaseModel):
    """
    schedule_execution_history
    
    ID fields: id
    """

    completed_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    duration_ms: Optional[int] = None
    error: Optional[str] = None
    job_id: str = Field(description="Reference: job.id")
    retry_attempt: int = Field(default=0)
    schedule_id: str = Field(description="Reference: schedule.id")
    scheduled_at: datetime
    started_at: Optional[datetime] = None
    status: str = Field(default="pending")
    worker_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class ScheduleRetryConfig(BaseModel):
    """
    schedule_retry_config
    
    ID fields: id
    """

    initial_delay_secs: int = Field(default=10)
    jitter: bool = Field(default=True)
    max_delay_secs: int = Field(default=3600)
    max_retries: int = Field(default=3)
    multiplier: float = Field(default="2.0")
    strategy: str = Field(default="exponential")
    class Config:
        from_attributes = True
        populate_by_name = True


class ScheduleBackoffStrategy(BaseModel):
    """
    schedule_backoff_strategy
    
    ID fields: id
    """

    value: str = Field(default="exponential")
    class Config:
        from_attributes = True
        populate_by_name = True


class ScheduleQuery(BaseModel):
    """
    schedule_query
    
    ID fields: id
    """

    due_before: Optional[datetime] = None
    limit: Optional[int] = None
    offset: Optional[int] = None
    owner_id: Optional[str] = None
    schedule_type: str
    search: Optional[str] = None
    status: str
    tags: Optional[List[str]] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class ScheduleStats(BaseModel):
    """
    schedule_stats
    
    ID fields: id
    """

    average_duration_ms: Optional[float] = None
    failed_executions: int = Field(default=0)
    last_execution_at: Optional[datetime] = None
    next_execution_at: Optional[datetime] = None
    successful_executions: int = Field(default=0)
    total_executions: int = Field(default=0)
    class Config:
        from_attributes = True
        populate_by_name = True


class ScheduleSchedulerConfig(BaseModel):
    """
    schedule_scheduler_config
    
    ID fields: id
    """

    auto_create_jobs: bool = Field(default=True)
    batch_size: int = Field(default=100)
    cleanup_interval_secs: int = Field(default=3600)
    history_retention_days: int = Field(default=30)
    lock_duration_secs: int = Field(default=300)
    max_concurrent_jobs: int = Field(default=10)
    poll_interval_secs: int = Field(default=10)
    stale_check_interval_secs: int = Field(default=60)
    stale_threshold_secs: int = Field(default=600)
    worker_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ScheduleInfo(BaseModel):
    """
    schedule_info
    
    ID fields: id
    """

    cron_expression: Optional[str] = None
    enabled: bool
    last_run: Optional[int] = None
    name: str
    next_run: Optional[int] = None
    schedule_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


