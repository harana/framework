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


class List(BaseModel):
    """
    list
    
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


class CheckPermission(BaseModel):
    """
    check_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddPermission(BaseModel):
    """
    add_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemovePermission(BaseModel):
    """
    remove_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Clone(BaseModel):
    """
    clone
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Role(BaseModel):
    """
    role
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RolePermission(BaseModel):
    """
    role_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RoleInfo(BaseModel):
    """
    role_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RolePermissionConditions(BaseModel):
    """
    role_permission_conditions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


