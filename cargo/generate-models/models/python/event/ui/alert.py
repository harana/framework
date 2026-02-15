# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AlertOpened(BaseModel):
    """
    alert_opened
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AlertClosed(BaseModel):
    """
    alert_closed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AlertActionClicked(BaseModel):
    """
    alert_action_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AlertDismissed(BaseModel):
    """
    alert_dismissed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


