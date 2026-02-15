# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class WebdriverXpathResult(BaseModel):
    """
    webdriver_xpath_result
    
    ID fields: session_id, xpath, queried_at
    """

    element_count: int = Field(default=0)
    error_message: Optional[str] = None
    queried_at: datetime = Field(default_factory=datetime.utcnow)
    results: Optional[str] = None
    session_id: str = Field(description="Reference: web_driver_session.id")
    status: str = Field(default="success")
    xpath: str
    class Config:
        from_attributes = True
        populate_by_name = True


