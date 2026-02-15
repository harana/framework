# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class LogEntry(BaseModel):
    """
    log_entry
    
    ID fields: source, created_at
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    level: str = Field(default="info")
    message: str
    metadata: Optional[str] = None
    request_id: Optional[str] = None
    source: Optional[str] = None
    span_id: Optional[str] = None
    trace_id: Optional[str] = None
    user_id: Optional[str] = Field(default=None, description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class LogError(BaseModel):
    """
    log_error
    
    ID fields: log_entry_id
    """

    code: Optional[str] = None
    log_entry_id: str = Field(description="Reference: log_entry.id")
    message: str
    stack_trace: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class LogConfig(BaseModel):
    """
    log_config
    
    ID fields: source
    """

    is_enabled: bool = Field(default=True)
    level: str = Field(default="info")
    max_retention_days: int = Field(default=30)
    source: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class LogContext(BaseModel):
    """
    log_context
    
    ID fields: id
    """

    request_id: str
    user_id: str
    trace_id: str
    span_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class LogData(BaseModel):
    """
    log_data
    
    ID fields: id
    """

    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


