# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class InputCleared(BaseModel):
    """
    input_cleared
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TextareaResized(BaseModel):
    """
    textarea_resized
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CharacterLimitReached(BaseModel):
    """
    character_limit_reached
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


