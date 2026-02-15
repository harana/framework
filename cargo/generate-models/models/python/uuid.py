# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class UuidRegistry(BaseModel):
    """
    uuid_registry
    
    ID fields: value
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    entity_id: Optional[str] = None
    entity_type: Optional[str] = None
    value: str
    variant: Optional[str] = None
    version: Optional[int] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class Uuid(BaseModel):
    """
    uuid
    
    ID fields: id
    """

    value: str
    version: int
    variant: str
    timestamp: int
    class Config:
        from_attributes = True
        populate_by_name = True


