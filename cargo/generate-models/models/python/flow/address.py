# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Validate(BaseModel):
    """
    validate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Normalize(BaseModel):
    """
    normalize
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Parse(BaseModel):
    """
    parse
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Autocomplete(BaseModel):
    """
    autocomplete
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Address(BaseModel):
    """
    address
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddressValidationError(BaseModel):
    """
    address_validation_error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddressSuggestion(BaseModel):
    """
    address_suggestion
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


