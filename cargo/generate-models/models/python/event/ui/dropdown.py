# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DropdownOpened(BaseModel):
    """
    dropdown_opened
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DropdownClosed(BaseModel):
    """
    dropdown_closed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class OptionSelected(BaseModel):
    """
    option_selected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class OptionDeselected(BaseModel):
    """
    option_deselected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SearchFiltered(BaseModel):
    """
    search_filtered
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ComboboxTyped(BaseModel):
    """
    combobox_typed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


