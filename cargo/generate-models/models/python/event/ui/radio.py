# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class RadioSelected(BaseModel):
    """
    radio_selected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RadioGroupChanged(BaseModel):
    """
    radio_group_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RadioFocused(BaseModel):
    """
    radio_focused
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RadioBlurred(BaseModel):
    """
    radio_blurred
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


