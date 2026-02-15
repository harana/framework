# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class WebdriverScriptExecution(BaseModel):
    """
    webdriver_script_execution
    
    ID fields: session_id, executed_at
    """

    arguments: Optional[str] = None
    async: bool = Field(default=False)
    duration_ms: Optional[int] = None
    error_message: Optional[str] = None
    executed_at: datetime = Field(default_factory=datetime.utcnow)
    result: Optional[str] = None
    script: str
    session_id: str = Field(description="Reference: web_driver_session.id")
    status: str = Field(default="success")
    class Config:
        from_attributes = True
        populate_by_name = True


