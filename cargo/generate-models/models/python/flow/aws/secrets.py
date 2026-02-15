# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Create(BaseModel):
    """
    create
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetValue(BaseModel):
    """
    get_value
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Update(BaseModel):
    """
    update
    
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


class Restore(BaseModel):
    """
    restore
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Lists(BaseModel):
    """
    lists
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Describe(BaseModel):
    """
    describe
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutValue(BaseModel):
    """
    put_value
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Rotate(BaseModel):
    """
    rotate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CancelRotate(BaseModel):
    """
    cancel_rotate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Tag(BaseModel):
    """
    tag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Untag(BaseModel):
    """
    untag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Replicate(BaseModel):
    """
    replicate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveRegionsFromReplication(BaseModel):
    """
    remove_regions_from_replication
    
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


