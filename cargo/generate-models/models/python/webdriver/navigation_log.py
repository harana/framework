# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class WebdriverNavigationLog(BaseModel):
    """
    webdriver_navigation_log
    
    ID fields: session_id, navigated_at
    """

    duration_ms: Optional[int] = None
    error_message: Optional[str] = None
    navigated_at: datetime = Field(default_factory=datetime.utcnow)
    page_title: Optional[str] = None
    session_id: str = Field(description="Reference: web_driver_session.id")
    status: str = Field(default="success")
    url: str
    class Config:
        from_attributes = True
        populate_by_name = True


