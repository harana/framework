# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Permission(BaseModel):
    """
    Permission
    
    Class: permission
    ID fields: name
    """

    action: str = Field(default="read")
    description: Optional[str] = None
    resource: str
    class Config:
        from_attributes = True
        populate_by_name = True


class Role(BaseModel):
    """
    Role
    
    Class: role
    ID fields: name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_system: bool = Field(default=False)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class RolePermission(BaseModel):
    """
    Role Permission
    
    Class: role_permission
    ID fields: role_id, permission_id
    """

    permission_id: str
    role_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class UserRole(BaseModel):
    """
    User Role
    
    Class: user_role
    ID fields: user_id, role_id
    """

    assigned_at: datetime = Field(default_factory=datetime.utcnow)
    assigned_by: Optional[str] = Field(default=None, description="Reference: User.id")
    role_id: str
    user_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


