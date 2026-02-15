# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class MenuOpened(BaseModel):
    """
    menu_opened
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MenuClosed(BaseModel):
    """
    menu_closed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SubmenuExpanded(BaseModel):
    """
    submenu_expanded
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


