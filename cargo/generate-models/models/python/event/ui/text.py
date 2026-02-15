# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class TextClicked(BaseModel):
    """
    text_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TextLinkClicked(BaseModel):
    """
    text_link_clicked
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TextSelected(BaseModel):
    """
    text_selected
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TextCopied(BaseModel):
    """
    text_copied
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CodeCopied(BaseModel):
    """
    code_copied
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


