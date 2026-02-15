# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SidebarToggled(BaseModel):
    """
    sidebar_toggled
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SidebarCollapsed(BaseModel):
    """
    sidebar_collapsed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SidebarExpanded(BaseModel):
    """
    sidebar_expanded
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


