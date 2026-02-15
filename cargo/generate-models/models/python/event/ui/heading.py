# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class HeadingClicked(BaseModel):
    """
    heading_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SubheadingClicked(BaseModel):
    """
    subheading_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


