# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class ShortUrl(BaseModel):
    """
    short_url
    
    ID fields: alias
    """

    alias: str
    click_count: int = Field(default=0)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    expires_at: Optional[datetime] = None
    is_active: bool = Field(default=True)
    original_url: str
    short_url: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ShortUrlClick(BaseModel):
    """
    short_url_click
    
    ID fields: short_url_id, clicked_at
    """

    clicked_at: datetime = Field(default_factory=datetime.utcnow)
    ip_address: Optional[str] = None
    referrer: Optional[str] = None
    short_url_id: str = Field(description="Reference: short_url.id")
    user_agent: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


