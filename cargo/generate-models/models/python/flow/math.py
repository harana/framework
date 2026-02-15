# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Round(BaseModel):
    """
    round
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Floor(BaseModel):
    """
    floor
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Ceil(BaseModel):
    """
    ceil
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Abs(BaseModel):
    """
    abs
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Min(BaseModel):
    """
    min
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Max(BaseModel):
    """
    max
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Sum(BaseModel):
    """
    sum
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Average(BaseModel):
    """
    average
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Percentage(BaseModel):
    """
    percentage
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class NumberValue(BaseModel):
    """
    number_value
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


