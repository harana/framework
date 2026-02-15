# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SwitchToggledOn(BaseModel):
    """
    switch_toggled_on
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SwitchToggledOff(BaseModel):
    """
    switch_toggled_off
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SwitchChanged(BaseModel):
    """
    switch_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SwitchGroupChanged(BaseModel):
    """
    switch_group_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


