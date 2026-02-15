# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class TextareaChanged(BaseModel):
    """
    textarea_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TextareaFocused(BaseModel):
    """
    textarea_focused
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TextareaBlurred(BaseModel):
    """
    textarea_blurred
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TextareaInput(BaseModel):
    """
    textarea_input
    
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


class TextareaCharacterLimitReached(BaseModel):
    """
    textarea_character_limit_reached
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


