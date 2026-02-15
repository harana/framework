# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class WebdriverElementInteraction(BaseModel):
    """
    webdriver_element_interaction
    
    ID fields: session_id, performed_at
    """

    action: str = Field(default="click")
    duration_ms: Optional[int] = None
    error_message: Optional[str] = None
    input_value: Optional[str] = None
    performed_at: datetime = Field(default_factory=datetime.utcnow)
    selector_type: str = Field(default="css")
    selector_value: str
    session_id: str = Field(description="Reference: web_driver_session.id")
    status: str = Field(default="success")
    class Config:
        from_attributes = True
        populate_by_name = True


