# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareBrowserRenderingSession(BaseModel):
    """
    cloudflare_browser_rendering_session
    
    ID fields: account_id, session_id
    """

    account_id: str
    binding: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    session_id: str
    status: str = Field(default="active")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareBrowserRenderingResult(BaseModel):
    """
    cloudflare_browser_rendering_result
    
    ID fields: session_id, rendered_at
    """

    content: Optional[str] = None
    content_type: str = Field(default="html")
    rendered_at: datetime = Field(default_factory=datetime.utcnow)
    session_id: str = Field(description="Reference: cf_browser_rendering_session.id")
    status_code: Optional[int] = None
    title: Optional[str] = None
    url: str
    class Config:
        from_attributes = True
        populate_by_name = True


