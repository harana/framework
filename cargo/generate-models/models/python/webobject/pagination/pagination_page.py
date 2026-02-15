# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class PaginationPage(BaseModel):
    """
    pagination_page
    
    ID fields: pagination_id, page_number
    """

    class Config:
        from_attributes = True
        populate_by_name = True


