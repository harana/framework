# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class PageChanged(BaseModel):
    """
    page_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PreviousPageClicked(BaseModel):
    """
    previous_page_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class NextPageClicked(BaseModel):
    """
    next_page_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PageNumberClicked(BaseModel):
    """
    page_number_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FirstPageReached(BaseModel):
    """
    first_page_reached
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class LastPageReached(BaseModel):
    """
    last_page_reached
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


