# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CalendarEvent(BaseModel):
    """
    calendar_event
    
    ID fields: id
    """

    id: str
    name: str
    datetime: str
    time_display: str
    href: Optional[str] = None
    location: Optional[str] = None
    color: str = Field(default=""blue"")
    grid_row_start: Optional[int] = None
    grid_row_span: Optional[int] = None
    day_column: Optional[int] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class CalendarMeeting(BaseModel):
    """
    calendar_meeting
    
    ID fields: id
    """

    id: str
    name: str
    image_url: Optional[str] = None
    datetime: str
    datetime_display: str
    location: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class CalendarDay(BaseModel):
    """
    calendar_day
    
    ID fields: id
    """

    date: str
    day_number: int
    weekday_short: Optional[str] = None
    weekday_long: Optional[str] = None
    is_current_month: bool = Field(default=False)
    is_today: bool = Field(default=False)
    is_selected: bool = Field(default=False)
    events: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class CalendarViewOption(BaseModel):
    """
    calendar_view_option
    
    ID fields: id
    """

    id: str
    label: str
    href: Optional[str] = None
    is_active: bool = Field(default=False)
    class Config:
        from_attributes = True
        populate_by_name = True


class CalendarMonth(BaseModel):
    """
    calendar_month
    
    ID fields: id
    """

    name: str
    days: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class TimeSlot(BaseModel):
    """
    time_slot
    
    ID fields: id
    """

    label: str
    hour: int
    class Config:
        from_attributes = True
        populate_by_name = True


