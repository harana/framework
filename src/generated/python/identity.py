# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Group(BaseModel):
    """
    Group
    
    Class: group
    ID fields: id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    id: str
    is_active: bool = Field(default=True)
    name: str
    parent_id: Optional[str] = Field(default=None, description="Reference: Group.id")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class GroupMember(BaseModel):
    """
    Group Member
    
    Class: group_member
    ID fields: group_id, user_id
    """

    added_at: datetime = Field(default_factory=datetime.utcnow)
    added_by: Optional[str] = Field(default=None, description="Reference: User.id")
    group_id: str
    is_admin: bool = Field(default=False)
    user_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class GroupRole(BaseModel):
    """
    Group Role
    
    Class: group_role
    ID fields: group_id, role_id
    """

    assigned_at: datetime = Field(default_factory=datetime.utcnow)
    assigned_by: Optional[str] = Field(default=None, description="Reference: User.id")
    group_id: str
    role_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class Role(BaseModel):
    """
    Role
    
    Class: role
    ID fields: id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    id: str
    is_system: bool = Field(default=False)
    name: str
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

    granted_at: datetime = Field(default_factory=datetime.utcnow)
    granted_by: Optional[str] = Field(default=None, description="Reference: User.id")
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
    expires_at: Optional[datetime] = None
    role_id: str
    user_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class User(BaseModel):
    """
    User
    
    Class: user
    ID fields: id
    """

    avatar_url: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    email: str
    first_name: Optional[str] = None
    id: str
    is_active: bool = Field(default=True)
    is_verified: bool = Field(default=False)
    last_login_at: Optional[datetime] = None
    last_name: Optional[str] = None
    phone: Optional[str] = None
    timezone: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    username: str
    class Config:
        from_attributes = True
        populate_by_name = True


class UserProfile(BaseModel):
    """
    User Profile
    
    Class: user_profile
    ID fields: user_id
    """

    bio: Optional[str] = None
    date_of_birth: Optional[str] = None
    job_title: Optional[str] = None
    location: Optional[str] = None
    organisation: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    user_id: str
    website: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class UserCredential(BaseModel):
    """
    User Credential
    
    Class: user_credential
    ID fields: user_id, credential_type
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    credential_type: str = Field(default="password")
    credential_value: str
    is_primary: bool = Field(default=False)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    user_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


