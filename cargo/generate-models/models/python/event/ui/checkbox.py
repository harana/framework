# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CheckboxChecked(BaseModel):
    """
    checkbox_checked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CheckboxUnchecked(BaseModel):
    """
    checkbox_unchecked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CheckboxIndeterminateChanged(BaseModel):
    """
    checkbox_indeterminate_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CheckboxGroupChanged(BaseModel):
    """
    checkbox_group_changed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


