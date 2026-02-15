# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CsvDocument(BaseModel):
    """
    csv_document
    
    ID fields: id
    """

    column_count: int = Field(default=0)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    delimiter: str = Field(default=",")
    has_headers: bool = Field(default=True)
    row_count: int = Field(default=0)
    size: int = Field(default=0)
    source_path: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class CsvColumnSchema(BaseModel):
    """
    csv_column_schema
    
    ID fields: document_id, name
    """

    document_id: str = Field(description="Reference: csv_document.id")
    is_required: bool = Field(default=False)
    name: str
    pattern: Optional[str] = None
    sort_order: int = Field(default=0)
    type: str = Field(default="string")
    class Config:
        from_attributes = True
        populate_by_name = True


class CsvValidationResult(BaseModel):
    """
    csv_validation_result
    
    ID fields: document_id, validated_at
    """

    document_id: str = Field(description="Reference: csv_document.id")
    error_count: int = Field(default=0)
    errors: Optional[str] = None
    is_valid: bool = Field(default=False)
    validated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


