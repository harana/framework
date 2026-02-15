# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Role(BaseModel):
    """
    role
    
    ID fields: id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_system: bool = Field(default=False)
    name: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class RolePermission(BaseModel):
    """
    role_permission
    
    ID fields: role_id, action, resource
    """

    action: str
    conditions: Optional[str] = None
    granted_at: datetime = Field(default_factory=datetime.utcnow)
    granted_by: Optional[str] = Field(default=None, description="Reference: user.id")
    resource: str
    role_id: str = Field(description="Reference: role.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class RoleAssignment(BaseModel):
    """
    role_assignment
    
    ID fields: role_id, entity_id, entity_type
    """

    assigned_at: datetime = Field(default_factory=datetime.utcnow)
    assigned_by: Optional[str] = Field(default=None, description="Reference: user.id")
    entity_id: str
    entity_type: str = Field(default="user")
    expires_at: Optional[datetime] = None
    role_id: str = Field(description="Reference: role.id")
    class Config:
        from_attributes = True
        populate_by_name = True


