# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CompressionJob(BaseModel):
    """
    compression_job
    
    ID fields: id
    """

    algorithm: str = Field(default="gzip")
    completed_at: Optional[datetime] = None
    compressed_size: Optional[int] = None
    compression_level: int = Field(default=6)
    compression_ratio: Optional[float] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    direction: str = Field(default="compress")
    error_message: Optional[str] = None
    original_size: Optional[int] = None
    status: str = Field(default="pending")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


