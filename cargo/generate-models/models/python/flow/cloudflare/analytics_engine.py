# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class WriteDataPoint(BaseModel):
    """
    write_data_point
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Query(BaseModel):
    """
    query
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


