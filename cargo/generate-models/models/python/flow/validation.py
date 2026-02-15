# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Schema(BaseModel):
    """
    schema
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Field(BaseModel):
    """
    field
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EmailFormat(BaseModel):
    """
    email_format
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Url(BaseModel):
    """
    url
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Phone(BaseModel):
    """
    phone
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Date(BaseModel):
    """
    date
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Json(BaseModel):
    """
    json
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreditCard(BaseModel):
    """
    credit_card
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Uuid(BaseModel):
    """
    uuid
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Ip(BaseModel):
    """
    ip
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Password(BaseModel):
    """
    password
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SanitizeHtml(BaseModel):
    """
    sanitize_html
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ValidationResult(BaseModel):
    """
    validation_result
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ValidationSchema(BaseModel):
    """
    validation_schema
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ValidationError(BaseModel):
    """
    validation_error
    
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


