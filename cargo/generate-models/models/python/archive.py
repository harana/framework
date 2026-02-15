# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Archive(BaseModel):
    """
    archive
    
    ID fields: id
    """

    archive_type: str = Field(default="zip")
    compression_level: int = Field(default=6)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    file_count: int = Field(default=0)
    is_encrypted: bool = Field(default=False)
    original_size: Optional[int] = None
    output_path: str
    size: Optional[int] = None
    status: str = Field(default="pending")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class ArchiveEntry(BaseModel):
    """
    archive_entry
    
    ID fields: archive_id, path
    """

    archive_id: str = Field(description="Reference: archive.id")
    compressed_size: Optional[int] = None
    is_directory: bool = Field(default=False)
    modified_at: Optional[datetime] = None
    original_size: Optional[int] = None
    path: str
    permissions: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


