# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DateSelected(BaseModel):
    """
    date_selected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DateRangeSelected(BaseModel):
    """
    date_range_selected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TimeSelected(BaseModel):
    """
    time_selected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CalendarMonthChanged(BaseModel):
    """
    calendar_month_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CalendarYearChanged(BaseModel):
    """
    calendar_year_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


