# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class NavbarItemClicked(BaseModel):
    """
    navbar_item_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class NavbarItemHovered(BaseModel):
    """
    navbar_item_hovered
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class NavbarSectionExpanded(BaseModel):
    """
    navbar_section_expanded
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class NavbarSectionCollapsed(BaseModel):
    """
    navbar_section_collapsed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class NavbarCurrentItemChanged(BaseModel):
    """
    navbar_current_item_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


