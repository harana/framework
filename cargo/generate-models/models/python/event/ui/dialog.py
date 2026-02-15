# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DialogOpened(BaseModel):
    """
    dialog_opened
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DialogClosed(BaseModel):
    """
    dialog_closed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DialogActionClicked(BaseModel):
    """
    dialog_action_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DialogSubmitted(BaseModel):
    """
    dialog_submitted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DialogCancelled(BaseModel):
    """
    dialog_cancelled
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


