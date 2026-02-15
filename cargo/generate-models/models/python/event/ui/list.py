# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class ListItemClicked(BaseModel):
    """
    list_item_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListItemReordered(BaseModel):
    """
    list_item_reordered
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListItemDeleted(BaseModel):
    """
    list_item_deleted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListItemArchived(BaseModel):
    """
    list_item_archived
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListFiltered(BaseModel):
    """
    list_filtered
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListGrouped(BaseModel):
    """
    list_grouped
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


