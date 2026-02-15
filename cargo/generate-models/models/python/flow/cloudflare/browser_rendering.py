# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Render(BaseModel):
    """
    render
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Screenshot(BaseModel):
    """
    screenshot
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExtractContent(BaseModel):
    """
    extract_content
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Pdf(BaseModel):
    """
    pdf
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


