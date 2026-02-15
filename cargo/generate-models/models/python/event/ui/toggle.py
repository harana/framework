# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class ToggleSwitched(BaseModel):
    """
    toggle_switched
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CheckboxChecked(BaseModel):
    """
    checkbox_checked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CheckboxUnchecked(BaseModel):
    """
    checkbox_unchecked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RadioSelected(BaseModel):
    """
    radio_selected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SwitchEnabled(BaseModel):
    """
    switch_enabled
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SwitchDisabled(BaseModel):
    """
    switch_disabled
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


