# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Image(BaseModel):
    """
    image
    
    ID fields: id
    """

    blob_id: Optional[str] = Field(default=None, description="Reference: blob_object.id")
    color_space: Optional[str] = None
    content_type: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    format: str = Field(default="jpeg")
    has_alpha: bool = Field(default=False)
    height: int
    orientation: int = Field(default=1)
    size: int
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    width: int
    class Config:
        from_attributes = True
        populate_by_name = True


class ImageVariant(BaseModel):
    """
    image_variant
    
    ID fields: image_id, variant_type
    """

    blob_id: Optional[str] = Field(default=None, description="Reference: blob_object.id")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    format: str = Field(default="webp")
    height: int
    image_id: str = Field(description="Reference: image.id")
    quality: int = Field(default=80)
    size: int
    variant_type: str = Field(default="original")
    width: int
    class Config:
        from_attributes = True
        populate_by_name = True


class ImageOperation(BaseModel):
    """
    image_operation
    
    ID fields: image_id, created_at
    """

    completed_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    error_message: Optional[str] = None
    image_id: str = Field(description="Reference: image.id")
    operation: str = Field(default="resize")
    parameters: Optional[str] = None
    status: str = Field(default="pending")
    class Config:
        from_attributes = True
        populate_by_name = True


