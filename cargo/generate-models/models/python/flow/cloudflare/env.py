# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

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


class GetStoreSecret(BaseModel):
    """
    get_store_secret
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListVars(BaseModel):
    """
    list_vars
    
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


