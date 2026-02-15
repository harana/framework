# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CommandPaletteOpened(BaseModel):
    """
    command_palette_opened
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CommandPaletteClosed(BaseModel):
    """
    command_palette_closed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CommandSelected(BaseModel):
    """
    command_selected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CommandSearched(BaseModel):
    """
    command_searched
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


