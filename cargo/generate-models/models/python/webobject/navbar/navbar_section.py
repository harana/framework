# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class NavbarSection(BaseModel):
    """
    navbar_section
    
    ID fields: navbar_id, name
    """

    class Config:
        from_attributes = True
        populate_by_name = True


