# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class JsonDocument(BaseModel):
    """
    json_document
    
    ID fields: id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    is_valid: bool = Field(default=True)
    schema_id: Optional[str] = Field(default=None, description="Reference: json_schema.id")
    size: int = Field(default=0)
    source_path: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class JsonSchema(BaseModel):
    """
    json_schema
    
    ID fields: id
    """

    additional_properties: bool = Field(default=True)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    properties: Optional[str] = None
    required_fields: Optional[str] = None
    schema_type: str = Field(default="object")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    version: int = Field(default=1)
    class Config:
        from_attributes = True
        populate_by_name = True


class JsonValidationResult(BaseModel):
    """
    json_validation_result
    
    ID fields: document_id, validated_at
    """

    document_id: str = Field(description="Reference: json_document.id")
    error_count: int = Field(default=0)
    errors: Optional[str] = None
    is_valid: bool = Field(default=False)
    schema_id: str = Field(description="Reference: json_schema.id")
    validated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


