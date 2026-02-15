# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Group(BaseModel):
    """
    group
    
    ID fields: id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    name: str
    parent_id: Optional[str] = Field(default=None, description="Reference: group.id")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class GroupMember(BaseModel):
    """
    group_member
    
    ID fields: group_id, user_id
    """

    added_at: datetime = Field(default_factory=datetime.utcnow)
    added_by: Optional[str] = Field(default=None, description="Reference: user.id")
    group_id: str = Field(description="Reference: group.id")
    is_admin: bool = Field(default=False)
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class GroupRole(BaseModel):
    """
    group_role
    
    ID fields: group_id, role_id
    """

    assigned_at: datetime = Field(default_factory=datetime.utcnow)
    assigned_by: Optional[str] = Field(default=None, description="Reference: user.id")
    group_id: str = Field(description="Reference: group.id")
    role_id: str = Field(description="Reference: role.id")
    class Config:
        from_attributes = True
        populate_by_name = True


