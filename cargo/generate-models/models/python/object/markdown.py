# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class MarkdownDocument(BaseModel):
    """
    markdown_document
    
    ID fields: id
    """

    content: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    frontmatter: Optional[str] = None
    is_published: bool = Field(default=False)
    slug: Optional[str] = None
    title: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


