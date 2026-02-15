# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class WebdriverScreenshot(BaseModel):
    """
    webdriver_screenshot
    
    ID fields: session_id, captured_at
    """

    captured_at: datetime = Field(default_factory=datetime.utcnow)
    element_selector: Optional[str] = None
    file_path: Optional[str] = None
    file_size: Optional[int] = None
    format: str = Field(default="png")
    is_element: bool = Field(default=False)
    session_id: str = Field(description="Reference: web_driver_session.id")
    url: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


