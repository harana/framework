# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class FieldFocused(BaseModel):
    """
    field_focused
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FieldBlurred(BaseModel):
    """
    field_blurred
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FieldChanged(BaseModel):
    """
    field_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


