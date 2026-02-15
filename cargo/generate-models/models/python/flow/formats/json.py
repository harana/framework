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


class Stringify(BaseModel):
    """
    stringify
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Validate(BaseModel):
    """
    validate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class JmespathQuery(BaseModel):
    """
    jmespath_query
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Merge(BaseModel):
    """
    merge
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Diff(BaseModel):
    """
    diff
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class JsonDocument(BaseModel):
    """
    json_document
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class JsonSchema(BaseModel):
    """
    json_schema
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class JsonValidationError(BaseModel):
    """
    json_validation_error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class JsonObject(BaseModel):
    """
    json_object
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


