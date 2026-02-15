# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class NotificationShown(BaseModel):
    """
    notification_shown
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class NotificationDismissed(BaseModel):
    """
    notification_dismissed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class NotificationClicked(BaseModel):
    """
    notification_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class NotificationActionClicked(BaseModel):
    """
    notification_action_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ToastExpired(BaseModel):
    """
    toast_expired
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


