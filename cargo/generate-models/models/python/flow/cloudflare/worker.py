# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Fetch(BaseModel):
    """
    fetch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetVar(BaseModel):
    """
    get_var
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetSecret(BaseModel):
    """
    get_secret
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Scheduled(BaseModel):
    """
    scheduled
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class WaitUntil(BaseModel):
    """
    wait_until
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PassThrough(BaseModel):
    """
    pass_through
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ServiceBindingFetch(BaseModel):
    """
    service_binding_fetch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetVersion(BaseModel):
    """
    get_version
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


