# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class RandomSeed(BaseModel):
    """
    random_seed
    
    ID fields: id
    """

    algorithm: str = Field(default="default")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    seed_value: Optional[int] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class RandomValue(BaseModel):
    """
    random_value
    
    ID fields: id
    """

    seed: int
    value: str
    type: str
    class Config:
        from_attributes = True
        populate_by_name = True


