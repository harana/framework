# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class XmlDocument(BaseModel):
    """
    xml_document
    
    ID fields: id
    """

    charset: str = Field(default="utf-8")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    root_element: Optional[str] = None
    size: int = Field(default=0)
    source_path: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class XmlSchema(BaseModel):
    """
    xml_schema
    
    ID fields: id
    """

    content: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    schema_type: str = Field(default="xsd")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    version: int = Field(default=1)
    class Config:
        from_attributes = True
        populate_by_name = True


class XmlValidationResult(BaseModel):
    """
    xml_validation_result
    
    ID fields: document_id, validated_at
    """

    document_id: str = Field(description="Reference: xml_document.id")
    error_count: int = Field(default=0)
    errors: Optional[str] = None
    is_valid: bool = Field(default=False)
    schema_id: str = Field(description="Reference: xml_schema.id")
    validated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class XmlObject(BaseModel):
    """
    xml_object
    
    ID fields: id
    """

    elements: str
    attributes: str
    class Config:
        from_attributes = True
        populate_by_name = True


class XmlValidationError(BaseModel):
    """
    xml_validation_error
    
    ID fields: id
    """

    line: int
    column: int
    message: str
    class Config:
        from_attributes = True
        populate_by_name = True


class XmlNamespaces(BaseModel):
    """
    xml_namespaces
    
    ID fields: id
    """

    namespaces: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class XmlNamespace(BaseModel):
    """
    xml_namespace
    
    ID fields: id
    """

    prefix: str
    uri: str
    class Config:
        from_attributes = True
        populate_by_name = True


