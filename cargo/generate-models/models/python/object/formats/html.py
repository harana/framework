# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class HtmlDocument(BaseModel):
    """
    html_document
    
    ID fields: id
    """

    charset: str = Field(default="utf-8")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    is_sanitized: bool = Field(default=False)
    size: int = Field(default=0)
    source_path: Optional[str] = None
    title: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


