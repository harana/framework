# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Exec(BaseModel):
    """
    exec
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Prepare(BaseModel):
    """
    prepare
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Run(BaseModel):
    """
    run
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class First(BaseModel):
    """
    first
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class All(BaseModel):
    """
    all
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Raw(BaseModel):
    """
    raw
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Batch(BaseModel):
    """
    batch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Dump(BaseModel):
    """
    dump
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


