# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Parse(BaseModel):
    """
    parse
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Sanitize(BaseModel):
    """
    sanitize
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExtractText(BaseModel):
    """
    extract_text
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CssSelect(BaseModel):
    """
    css_select
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Minify(BaseModel):
    """
    minify
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HtmlDocument(BaseModel):
    """
    html_document
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HtmlDom(BaseModel):
    """
    html_dom
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HtmlElement(BaseModel):
    """
    html_element
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HtmlNode(BaseModel):
    """
    html_node
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HtmlAllowedAttributes(BaseModel):
    """
    html_allowed_attributes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


