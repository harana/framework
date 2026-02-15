# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SliderChanged(BaseModel):
    """
    slider_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SliderDragStarted(BaseModel):
    """
    slider_drag_started
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SliderDragEnded(BaseModel):
    """
    slider_drag_ended
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RangeAdjusted(BaseModel):
    """
    range_adjusted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


