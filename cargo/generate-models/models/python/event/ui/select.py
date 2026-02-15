# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SelectOpened(BaseModel):
    """
    select_opened
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SelectClosed(BaseModel):
    """
    select_closed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SelectOptionSelected(BaseModel):
    """
    select_option_selected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SelectValueChanged(BaseModel):
    """
    select_value_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SelectFocused(BaseModel):
    """
    select_focused
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SelectBlurred(BaseModel):
    """
    select_blurred
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


