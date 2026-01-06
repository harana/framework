# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Healthcheck(BaseModel):
    """
    Healthcheck
    
    Class: healthcheck
    ID fields: name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    interval_seconds: int = Field(default=60)
    name: str
    timeout_seconds: int = Field(default=30)
    type: str = Field(default="route")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthcheckRoute(BaseModel):
    """
    Healthcheck Route
    
    Class: healthcheck_route
    ID fields: healthcheck_id
    """

    expected_status_codes: str = Field(default=200)
    expected_body_contains: Optional[str] = None
    headers: Optional[str] = None
    healthcheck_id: str
    method: str = Field(default="GET")
    url: str
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthcheckResponseTime(BaseModel):
    """
    Healthcheck Response Time
    
    Class: healthcheck_response_time
    ID fields: healthcheck_id
    """

    healthcheck_id: str
    route_id: Optional[str] = Field(default=None, description="Reference: Route.id")
    threshold_ms: int = Field(default=1000)
    url: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthcheckJobExecution(BaseModel):
    """
    Healthcheck Job Execution
    
    Class: healthcheck_job_execution
    ID fields: healthcheck_id
    """

    healthcheck_id: str
    job_name: str
    max_duration_seconds: Optional[int] = None
    max_failures_percent: Optional[float] = None
    queue: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthcheckEventRate(BaseModel):
    """
    Healthcheck Event Rate
    
    Class: healthcheck_event_rate
    ID fields: healthcheck_id
    """

    comparison: str = Field(default="gte")
    event_type: str
    healthcheck_id: str
    source: Optional[str] = None
    threshold: float
    window: str = Field(default="minute")
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthcheckCustom(BaseModel):
    """
    Healthcheck Custom
    
    Class: healthcheck_custom
    ID fields: healthcheck_id
    """

    healthcheck_id: str
    metric_id: Optional[str] = Field(default=None, description="Reference: Metric.id")
    query: str
    success_condition: str
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthcheckResult(BaseModel):
    """
    Healthcheck Result
    
    Class: healthcheck_result
    ID fields: healthcheck_id, checked_at
    """

    checked_at: datetime = Field(default_factory=datetime.utcnow)
    duration_ms: Optional[int] = None
    error_message: Optional[str] = None
    healthcheck_id: str
    metadata: Optional[str] = None
    status: str = Field(default="healthy")
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthcheckAlert(BaseModel):
    """
    Healthcheck Alert
    
    Class: healthcheck_alert
    ID fields: healthcheck_id
    """

    consecutive_failures: int = Field(default=3)
    cooldown_seconds: int = Field(default=300)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    healthcheck_id: str
    is_active: bool = Field(default=True)
    last_triggered_at: Optional[datetime] = None
    notification_channel: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


