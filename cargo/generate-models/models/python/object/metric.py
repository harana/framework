# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Metric(BaseModel):
    """
    metric
    
    ID fields: id
    """

    description: Optional[str] = None
    labels: Optional[str] = None
    type: str = Field(default="gauge")
    unit: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class MetricValue(BaseModel):
    """
    metric_value
    
    ID fields: metric_id, timestamp, labels
    """

    labels: Optional[str] = None
    metric_id: str = Field(description="Reference: metric.id")
    timestamp: datetime = Field(default_factory=datetime.utcnow)
    value: float
    class Config:
        from_attributes = True
        populate_by_name = True


class MetricAlert(BaseModel):
    """
    metric_alert
    
    ID fields: id
    """

    comparison: str = Field(default="gt")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    is_active: bool = Field(default=True)
    last_triggered_at: Optional[datetime] = None
    metric_id: str = Field(description="Reference: metric.id")
    notification_channel: Optional[str] = None
    threshold: float
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


