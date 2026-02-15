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


class GetWithMetadata(BaseModel):
    """
    get_with_metadata
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Put(BaseModel):
    """
    put
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutWithMetadata(BaseModel):
    """
    put_with_metadata
    
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


class List(BaseModel):
    """
    list
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


