# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DateFormat(BaseModel):
    """
    date_format
    
    ID fields: id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    format_string: str
    is_active: bool = Field(default=True)
    locale: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class Timezone(BaseModel):
    """
    timezone
    
    ID fields: name
    """

    abbreviation: Optional[str] = None
    dst_offset: int = Field(default=0)
    name: str
    utc_offset: int = Field(default=0)
    class Config:
        from_attributes = True
        populate_by_name = True


