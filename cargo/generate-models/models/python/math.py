# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class MathExpression(BaseModel):
    """
    math_expression
    
    ID fields: id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    expression: str
    result: Optional[float] = None
    variables: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class NumberValue(BaseModel):
    """
    number_value
    
    ID fields: id
    """

    value: float
    precision: int
    formatted: str
    class Config:
        from_attributes = True
        populate_by_name = True


