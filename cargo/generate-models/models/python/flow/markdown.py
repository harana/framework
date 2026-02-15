# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class ToHtml(BaseModel):
    """
    to_html
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FromHtml(BaseModel):
    """
    from_html
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExtractFrontmatter(BaseModel):
    """
    extract_frontmatter
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MarkdownDocument(BaseModel):
    """
    markdown_document
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MarkdownFrontmatter(BaseModel):
    """
    markdown_frontmatter
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


