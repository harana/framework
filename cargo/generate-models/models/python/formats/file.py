# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class FileDocument(BaseModel):
    """
    file_document
    
    ID fields: id
    """

    content_type: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    is_directory: bool = Field(default=False)
    modified_at: Optional[datetime] = None
    path: str
    permissions: Optional[str] = None
    size: int = Field(default=0)
    class Config:
        from_attributes = True
        populate_by_name = True


class File(BaseModel):
    """
    file
    
    ID fields: id
    """

    path: str
    name: str
    size: int
    content_type: str
    created: datetime
    modified: datetime
    is_directory: bool
    permissions: str
    class Config:
        from_attributes = True
        populate_by_name = True


