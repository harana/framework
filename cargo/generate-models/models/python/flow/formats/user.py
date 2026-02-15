# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CheckPermission(BaseModel):
    """
    check_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GrantPermission(BaseModel):
    """
    grant_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RevokePermission(BaseModel):
    """
    revoke_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListPermissions(BaseModel):
    """
    list_permissions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AssignRole(BaseModel):
    """
    assign_role
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveRole(BaseModel):
    """
    remove_role
    
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


class GetEffectivePermissions(BaseModel):
    """
    get_effective_permissions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UserPermission(BaseModel):
    """
    user_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UserPermissionConditions(BaseModel):
    """
    user_permission_conditions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UserPermissionInfo(BaseModel):
    """
    user_permission_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UserRoleInfo(BaseModel):
    """
    user_role_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EffectivePermission(BaseModel):
    """
    effective_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PermissionSource(BaseModel):
    """
    permission_source
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


