# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class ModalOpened(BaseModel):
    """
    modal_opened
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModalClosed(BaseModel):
    """
    modal_closed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModalDismissed(BaseModel):
    """
    modal_dismissed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ConfirmationAccepted(BaseModel):
    """
    confirmation_accepted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ConfirmationRejected(BaseModel):
    """
    confirmation_rejected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModalBackdropClicked(BaseModel):
    """
    modal_backdrop_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


