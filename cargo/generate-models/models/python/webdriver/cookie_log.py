# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class WebdriverCookieLog(BaseModel):
    """
    webdriver_cookie_log
    
    ID fields: session_id, action, performed_at
    """

    action: str = Field(default="get")
    cookie_domain: Optional[str] = None
    cookie_name: Optional[str] = None
    cookie_value: Optional[str] = None
    performed_at: datetime = Field(default_factory=datetime.utcnow)
    session_id: str = Field(description="Reference: web_driver_session.id")
    class Config:
        from_attributes = True
        populate_by_name = True


