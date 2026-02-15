# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class ButtonClicked(BaseModel):
    """
    button_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ButtonDoubleClicked(BaseModel):
    """
    button_double_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ButtonLongPressed(BaseModel):
    """
    button_long_pressed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ButtonDisabledClicked(BaseModel):
    """
    button_disabled_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


