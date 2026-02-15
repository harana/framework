# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Healthcheck(BaseModel):
    """
    healthcheck
    
    ID fields: id
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
    healthcheck_route
    
    ID fields: healthcheck_id
    """

    expected_status_codes: str = Field(default=200)
    expected_body_contains: Optional[str] = None
    headers: Optional[str] = None
    healthcheck_id: str = Field(description="Reference: healthcheck.id")
    method: str = Field(default="get")
    url: str
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthcheckResponseTime(BaseModel):
    """
    healthcheck_response_time
    
    ID fields: healthcheck_id
    """

    healthcheck_id: str = Field(description="Reference: healthcheck.id")
    route_id: Optional[str] = Field(default=None, description="Reference: route.id")
    threshold_ms: int = Field(default=1000)
    url: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthcheckJobExecution(BaseModel):
    """
    healthcheck_job_execution
    
    ID fields: healthcheck_id
    """

    healthcheck_id: str = Field(description="Reference: healthcheck.id")
    job_name: str
    max_duration_seconds: Optional[int] = None
    max_failures_percent: Optional[float] = None
    queue: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthcheckEventRate(BaseModel):
    """
    healthcheck_event_rate
    
    ID fields: healthcheck_id
    """

    comparison: str = Field(default="gte")
    event_type: str
    healthcheck_id: str = Field(description="Reference: healthcheck.id")
    source: Optional[str] = None
    threshold: float
    window: str = Field(default="minute")
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthcheckCustom(BaseModel):
    """
    healthcheck_custom
    
    ID fields: healthcheck_id
    """

    healthcheck_id: str = Field(description="Reference: healthcheck.id")
    metric_id: Optional[str] = Field(default=None, description="Reference: metric.id")
    query: str
    success_condition: str
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthcheckResult(BaseModel):
    """
    healthcheck_result
    
    ID fields: healthcheck_id, checked_at
    """

    checked_at: datetime = Field(default_factory=datetime.utcnow)
    duration_ms: Optional[int] = None
    error_message: Optional[str] = None
    healthcheck_id: str = Field(description="Reference: healthcheck.id")
    metadata: Optional[str] = None
    status: str = Field(default="healthy")
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthcheckAlert(BaseModel):
    """
    healthcheck_alert
    
    ID fields: healthcheck_id
    """

    consecutive_failures: int = Field(default=3)
    cooldown_seconds: int = Field(default=300)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    healthcheck_id: str = Field(description="Reference: healthcheck.id")
    is_active: bool = Field(default=True)
    last_triggered_at: Optional[datetime] = None
    notification_channel: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthStatus(BaseModel):
    """
    health_status
    
    ID fields: id
    """

    service_name: str
    healthy: bool
    status: str
    latency_ms: int
    details: str
    checked_at: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthDetails(BaseModel):
    """
    health_details
    
    ID fields: id
    """

    message: str
    error: str
    extra: str
    class Config:
        from_attributes = True
        populate_by_name = True


class HealthCheck(BaseModel):
    """
    health_check
    
    ID fields: id
    """

    name: str
    healthy: bool
    latency_ms: int
    error: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SystemMetrics(BaseModel):
    """
    system_metrics
    
    ID fields: id
    """

    cpu_usage: float
    memory_usage: float
    disk_usage: float
    active_connections: int
    class Config:
        from_attributes = True
        populate_by_name = True


