# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class TextTemplate(BaseModel):
    """
    text_template
    
    ID fields: id
    """

    content: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    engine: str = Field(default="handlebars")
    is_active: bool = Field(default=True)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    variables: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class TextString(BaseModel):
    """
    text_string
    
    ID fields: id
    """

    content: str
    length: int
    encoding: str
    class Config:
        from_attributes = True
        populate_by_name = True


class TemplateData(BaseModel):
    """
    template_data
    
    ID fields: id
    """

    values: str
    class Config:
        from_attributes = True
        populate_by_name = True


