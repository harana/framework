# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class App(BaseModel):
    """
    app
    
    ID fields: id
    """

    name: str
    description: Optional[str] = None
    commands: str
    defaults: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


