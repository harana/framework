# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class HttpResponseTemplate(BaseModel):
    """
    http_response_template
    
    ID fields: id
    """

    body_template: Optional[str] = None
    content_type: str = Field(default="application/json")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    headers: Optional[str] = None
    is_active: bool = Field(default=True)
    status_code: int = Field(default=200)
    template_engine: str = Field(default="handlebars")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class HttpResponseLog(BaseModel):
    """
    http_response_log
    
    ID fields: request_path, created_at
    """

    content_type: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    duration_ms: Optional[int] = None
    request_method: str = Field(default="get")
    request_path: str
    response_size: Optional[int] = None
    status_code: int
    template_id: Optional[str] = Field(default=None, description="Reference: http_response_template.id")
    class Config:
        from_attributes = True
        populate_by_name = True


