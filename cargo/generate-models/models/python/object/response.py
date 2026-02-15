# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class ResponseTemplate(BaseModel):
    """
    response_template
    
    ID fields: id
    """

    body_template: Optional[str] = None
    content_type: str = Field(default="application/json")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    headers: Optional[str] = None
    is_active: bool = Field(default=True)
    name: str
    status_code: int = Field(default=200)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class ResponseLog(BaseModel):
    """
    response_log
    
    ID fields: request_path, created_at
    """

    content_type: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    duration_ms: Optional[int] = None
    request_method: str
    request_path: str
    response_size: Optional[int] = None
    status_code: int
    template_id: Optional[str] = Field(default=None, description="Reference: response_template.id")
    class Config:
        from_attributes = True
        populate_by_name = True


