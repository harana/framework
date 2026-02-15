# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AuditEvent(BaseModel):
    """
    audit_event
    
    ID fields: actor_id, resource_id, created_at
    """

    action: str
    actor_id: str
    actor_type: str = Field(default="user")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    details: Optional[str] = None
    ip_address: Optional[str] = None
    metadata: Optional[str] = None
    outcome: str = Field(default="success")
    resource_id: str
    resource_type: str
    user_agent: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AuditAlert(BaseModel):
    """
    audit_alert
    
    ID fields: id
    """

    action_pattern: Optional[str] = None
    actor_pattern: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    last_triggered_at: Optional[datetime] = None
    notification_channel: Optional[str] = None
    resource_pattern: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AuditExport(BaseModel):
    """
    audit_export
    
    ID fields: created_at
    """

    completed_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    download_url: Optional[str] = None
    end_time: datetime
    expires_at: Optional[datetime] = None
    file_size: Optional[int] = None
    format: str = Field(default="json")
    record_count: Optional[int] = None
    start_time: datetime
    status: str = Field(default="pending")
    class Config:
        from_attributes = True
        populate_by_name = True


