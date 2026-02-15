# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class PdfDocument(BaseModel):
    """
    pdf_document
    
    ID fields: id
    """

    author: Optional[str] = None
    blob_id: Optional[str] = Field(default=None, description="Reference: blob_object.id")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    creator: Optional[str] = None
    is_encrypted: bool = Field(default=False)
    keywords: Optional[str] = None
    modification_date: Optional[datetime] = None
    page_count: int = Field(default=0)
    producer: Optional[str] = None
    size: int = Field(default=0)
    subject: Optional[str] = None
    title: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class PdfTemplate(BaseModel):
    """
    pdf_template
    
    ID fields: id
    """

    content_template: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    footer_template: Optional[str] = None
    header_template: Optional[str] = None
    is_active: bool = Field(default=True)
    orientation: str = Field(default="portrait")
    page_size: str = Field(default="a4")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class PdfGenerationLog(BaseModel):
    """
    pdf_generation_log
    
    ID fields: created_at
    """

    completed_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    error_message: Optional[str] = None
    output_size: Optional[int] = None
    page_count: Optional[int] = None
    source: str = Field(default="html")
    status: str = Field(default="pending")
    template_id: Optional[str] = Field(default=None, description="Reference: pdf_template.id")
    class Config:
        from_attributes = True
        populate_by_name = True


