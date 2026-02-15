# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateEvent(BaseModel):
    """
    create_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateEvent(BaseModel):
    """
    update_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteEvent(BaseModel):
    """
    delete_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetEvent(BaseModel):
    """
    get_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListEvents(BaseModel):
    """
    list_events
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateCalendar(BaseModel):
    """
    create_calendar
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteCalendar(BaseModel):
    """
    delete_calendar
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListCalendars(BaseModel):
    """
    list_calendars
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CheckAvailability(BaseModel):
    """
    check_availability
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddAttendee(BaseModel):
    """
    add_attendee
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveAttendee(BaseModel):
    """
    remove_attendee
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RespondToInvitation(BaseModel):
    """
    respond_to_invitation
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FindAvailableSlots(BaseModel):
    """
    find_available_slots
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CalendarEvent(BaseModel):
    """
    calendar_event
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Calendar(BaseModel):
    """
    calendar
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CalendarAttendee(BaseModel):
    """
    calendar_attendee
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CalendarReminder(BaseModel):
    """
    calendar_reminder
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CalendarTimePeriod(BaseModel):
    """
    calendar_time_period
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CalendarTimeSlot(BaseModel):
    """
    calendar_time_slot
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


