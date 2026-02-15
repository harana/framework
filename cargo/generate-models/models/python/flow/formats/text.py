# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Template(BaseModel):
    """
    template
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RegexMatch(BaseModel):
    """
    regex_match
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RegexReplace(BaseModel):
    """
    regex_replace
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Split(BaseModel):
    """
    split
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Join(BaseModel):
    """
    join
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Trim(BaseModel):
    """
    trim
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CaseConvert(BaseModel):
    """
    case_convert
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Truncate(BaseModel):
    """
    truncate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Slugify(BaseModel):
    """
    slugify
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TextString(BaseModel):
    """
    text_string
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TemplateData(BaseModel):
    """
    template_data
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


