# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class TransformJob(BaseModel):
    """
    transform_job
    
    ID fields: id
    """

    completed_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    error_message: Optional[str] = None
    input_format: str
    input_size: Optional[int] = None
    output_format: str
    output_size: Optional[int] = None
    status: str = Field(default="pending")
    transform_type: str = Field(default="json_to_xml")
    class Config:
        from_attributes = True
        populate_by_name = True


