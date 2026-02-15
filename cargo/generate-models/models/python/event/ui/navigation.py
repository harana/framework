# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class NavItemClicked(BaseModel):
    """
    nav_item_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TabSwitched(BaseModel):
    """
    tab_switched
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BreadcrumbClicked(BaseModel):
    """
    breadcrumb_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


