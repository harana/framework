# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Get(BaseModel):
    """
    get
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Set(BaseModel):
    """
    set
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Delete(BaseModel):
    """
    delete
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Exists(BaseModel):
    """
    exists
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Clear(BaseModel):
    """
    clear
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetMany(BaseModel):
    """
    get_many
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetMany(BaseModel):
    """
    set_many
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Increment(BaseModel):
    """
    increment
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Decrement(BaseModel):
    """
    decrement
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Ttl(BaseModel):
    """
    ttl
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CacheEntry(BaseModel):
    """
    cache_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CacheValues(BaseModel):
    """
    cache_values
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CacheEntries(BaseModel):
    """
    cache_entries
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


