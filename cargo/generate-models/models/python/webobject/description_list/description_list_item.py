# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DescriptionListItem(BaseModel):
    """
    description_list_item
    
    ID fields: list_id, term
    """

    class Config:
        from_attributes = True
        populate_by_name = True


