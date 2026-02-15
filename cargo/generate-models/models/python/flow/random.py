# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Uuid(BaseModel):
    """
    uuid
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Bytes(BaseModel):
    """
    bytes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class String(BaseModel):
    """
    string
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Number(BaseModel):
    """
    number
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Choice(BaseModel):
    """
    choice
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Shuffle(BaseModel):
    """
    shuffle
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RandomValue(BaseModel):
    """
    random_value
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


