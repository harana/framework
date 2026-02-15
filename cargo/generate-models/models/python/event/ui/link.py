# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class LinkClicked(BaseModel):
    """
    link_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class LinkHovered(BaseModel):
    """
    link_hovered
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class LinkFocused(BaseModel):
    """
    link_focused
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


