# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class ListboxOpened(BaseModel):
    """
    listbox_opened
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListboxClosed(BaseModel):
    """
    listbox_closed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListboxOptionSelected(BaseModel):
    """
    listbox_option_selected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListboxOptionHighlighted(BaseModel):
    """
    listbox_option_highlighted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListboxSearched(BaseModel):
    """
    listbox_searched
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


