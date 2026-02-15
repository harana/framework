# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class WebdriverSession(BaseModel):
    """
    webdriver_session
    
    ID fields: id
    """

    browser: str = Field(default="chrome")
    capabilities: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    headless: bool = Field(default=False)
    implicit_wait_ms: int = Field(default=0)
    is_active: bool = Field(default=True)
    page_load_timeout_ms: int = Field(default=60000)
    script_timeout_ms: int = Field(default=60000)
    server_url: str
    session_id: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    user_agent: Optional[str] = None
    window_height: int = Field(default=900)
    window_width: int = Field(default=1440)
    class Config:
        from_attributes = True
        populate_by_name = True


