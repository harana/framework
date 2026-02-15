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


class Get(BaseModel):
    """
    get
    
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


class Destroy(BaseModel):
    """
    destroy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Refresh(BaseModel):
    """
    refresh
    
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


class SetValue(BaseModel):
    """
    set_value
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteValue(BaseModel):
    """
    delete_value
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListUsers(BaseModel):
    """
    list_users
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DestroyUsers(BaseModel):
    """
    destroy_users
    
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


class Session(BaseModel):
    """
    session
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SessionData(BaseModel):
    """
    session_data
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SessionInfo(BaseModel):
    """
    session_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


