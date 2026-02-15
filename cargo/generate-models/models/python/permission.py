# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Permission(BaseModel):
    """
    permission
    
    ID fields: id
    """

    method: str = Field(default="read")
    description: Optional[str] = None
    resource: str
    class Config:
        from_attributes = True
        populate_by_name = True


class Role(BaseModel):
    """
    role
    
    ID fields: id
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
    role_permission
    
    ID fields: role_id, permission_id
    """

    permission_id: str = Field(description="Reference: permission.id")
    role_id: str = Field(description="Reference: role.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class UserRole(BaseModel):
    """
    user_role
    
    ID fields: user_id, role_id
    """

    assigned_at: datetime = Field(default_factory=datetime.utcnow)
    assigned_by: Optional[str] = Field(default=None, description="Reference: user.id")
    role_id: str = Field(description="Reference: role.id")
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


