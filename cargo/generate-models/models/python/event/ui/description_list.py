# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DescriptionListItemClicked(BaseModel):
    """
    description_list_item_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescriptionListItemExpanded(BaseModel):
    """
    description_list_item_expanded
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescriptionListItemCollapsed(BaseModel):
    """
    description_list_item_collapsed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


