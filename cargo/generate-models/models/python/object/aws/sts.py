# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsStsAssumedRoleSession(BaseModel):
    """
    aws_sts_assumed_role_session
    
    ID fields: account_id, role_arn, role_session_name
    """

    access_key_id: Optional[str] = None
    account_id: str
    assumed_role_arn: Optional[str] = None
    assumed_role_id: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    expiration: datetime
    method: str = Field(default="assume_role")
    packed_policy_size: Optional[int] = None
    region: Optional[str] = None
    role_arn: str
    role_session_name: str
    source_identity: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsStsFederationToken(BaseModel):
    """
    aws_sts_federation_token
    
    ID fields: account_id, federated_user_arn
    """

    access_key_id: Optional[str] = None
    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    expiration: datetime
    federated_user_arn: str
    federated_user_id: Optional[str] = None
    name: str
    packed_policy_size: Optional[int] = None
    region: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


