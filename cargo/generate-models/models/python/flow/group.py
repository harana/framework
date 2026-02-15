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


class Get(BaseModel):
    """
    get
    
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


class AddUserTo(BaseModel):
    """
    add_user_to
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveUserFrom(BaseModel):
    """
    remove_user_from
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListMembers(BaseModel):
    """
    list_members
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AssignRoleTo(BaseModel):
    """
    assign_role_to
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveRoleFrom(BaseModel):
    """
    remove_role_from
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListRoles(BaseModel):
    """
    list_roles
    
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


class Group(BaseModel):
    """
    group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GroupRole(BaseModel):
    """
    group_role
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


