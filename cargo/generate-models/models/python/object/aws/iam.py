# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsIamUser(BaseModel):
    """
    aws_iam_user
    
    ID fields: account_id, user_name
    """

    account_id: str
    arn: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    password_last_used: Optional[datetime] = None
    path: str = Field(default="/")
    permissions_boundary_arn: Optional[str] = None
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    user_id: Optional[str] = None
    user_name: str
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsIamGroup(BaseModel):
    """
    aws_iam_group
    
    ID fields: account_id, group_name
    """

    account_id: str
    arn: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    group_id: Optional[str] = None
    group_name: str
    path: str = Field(default="/")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsIamGroupMembership(BaseModel):
    """
    aws_iam_group_membership
    
    ID fields: group_id, user_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    group_id: str = Field(description="Reference: aws_iam_group.id")
    user_id: str = Field(description="Reference: aws_iamuser.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsIamRole(BaseModel):
    """
    aws_iam_role
    
    ID fields: account_id, role_name
    """

    account_id: str
    arn: Optional[str] = None
    assume_role_policy_document: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    max_session_duration: int = Field(default=3600)
    path: str = Field(default="/")
    permissions_boundary_arn: Optional[str] = None
    role_id: Optional[str] = None
    role_name: str
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsIamPolicy(BaseModel):
    """
    aws_iam_policy
    
    ID fields: account_id, policy_arn
    """

    account_id: str
    arn: Optional[str] = None
    attachment_count: int = Field(default=0)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    default_version_id: Optional[str] = None
    description: Optional[str] = None
    is_attachable: bool = Field(default=True)
    path: str = Field(default="/")
    policy_arn: str
    policy_id: Optional[str] = None
    policy_name: str
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsIamPolicyAttachment(BaseModel):
    """
    aws_iam_policy_attachment
    
    ID fields: policy_id, entity_type, entity_name
    """

    attached_at: datetime = Field(default_factory=datetime.utcnow)
    entity_name: str
    entity_type: str
    policy_id: str = Field(description="Reference: aws_iam_policy.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsIamAccessKey(BaseModel):
    """
    aws_iam_access_key
    
    ID fields: user_id, access_key_id
    """

    access_key_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    last_used_at: Optional[datetime] = None
    last_used_region: Optional[str] = None
    last_used_service: Optional[str] = None
    status: str = Field(default="active")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    user_id: str = Field(description="Reference: aws_iamuser.id")
    class Config:
        from_attributes = True
        populate_by_name = True


