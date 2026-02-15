# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class HtmlDocument(BaseModel):
    """
    html_document
    
    ID fields: id
    """

    charset: str = Field(default="utf-8")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    is_sanitized: bool = Field(default=False)
    size: int = Field(default=0)
    source_path: Optional[str] = None
    title: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class HtmlDom(BaseModel):
    """
    html_dom
    
    ID fields: id
    """

    document_type: str
    root: str
    class Config:
        from_attributes = True
        populate_by_name = True


class HtmlElement(BaseModel):
    """
    html_element
    
    ID fields: id
    """

    tag_name: str
    attributes: str
    children: List[str]
    text_content: str
    class Config:
        from_attributes = True
        populate_by_name = True


class HtmlNode(BaseModel):
    """
    html_node
    
    ID fields: id
    """

    node_type: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class HtmlAllowedAttributes(BaseModel):
    """
    html_allowed_attributes
    
    ID fields: id
    """

    global: List[str]
    per_tag: str
    class Config:
        from_attributes = True
        populate_by_name = True


