# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsSecretsManagerSecret(BaseModel):
    """
    aws_secrets_manager_secret
    
    ID fields: account_id, name
    """

    account_id: str
    arn: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    deleted_at: Optional[datetime] = None
    description: Optional[str] = None
    kms_key_id: Optional[str] = None
    last_changed_at: Optional[datetime] = None
    last_rotated_at: Optional[datetime] = None
    name: str
    region: Optional[str] = None
    rotation_enabled: bool = Field(default=False)
    rotation_lambda_arn: Optional[str] = None
    rotation_rules: Optional[str] = None
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsSecretsManagerSecretVersion(BaseModel):
    """
    aws_secrets_manager_secret_version
    
    ID fields: secret_id, version_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    secret_id: str = Field(description="Reference: aws_secrets_manager_secret.id")
    version_id: str
    version_stages: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsSecretsManagerSecretReplication(BaseModel):
    """
    aws_secrets_manager_secret_replication
    
    ID fields: secret_id, region
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    kms_key_id: Optional[str] = None
    last_accessed_at: Optional[datetime] = None
    region: str
    secret_id: str = Field(description="Reference: aws_secrets_manager_secret.id")
    status: str = Field(default="in_progress")
    status_message: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsSecretsManagerAccessLog(BaseModel):
    """
    aws_secrets_manager_access_log
    
    ID fields: secret_id, accessed_at
    """

    accessed_at: datetime = Field(default_factory=datetime.utcnow)
    action: str = Field(default="get_value")
    ip_address: Optional[str] = None
    secret_id: str = Field(description="Reference: aws_secrets_manager_secret.id")
    success: bool = Field(default=True)
    user_arn: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


