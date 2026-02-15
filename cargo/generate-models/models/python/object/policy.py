# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Policy(BaseModel):
    """
    policy
    
    ID fields: id
    """

    actions: str
    conditions: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    effect: str = Field(default="allow")
    is_active: bool = Field(default=True)
    name: str
    resources: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class PolicyAttachment(BaseModel):
    """
    policy_attachment
    
    ID fields: policy_id, entity_id, entity_type
    """

    attached_at: datetime = Field(default_factory=datetime.utcnow)
    attached_by: Optional[str] = Field(default=None, description="Reference: user.id")
    entity_id: str
    entity_type: str = Field(default="user")
    policy_id: str = Field(description="Reference: policy.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class PolicyEvaluationLog(BaseModel):
    """
    policy_evaluation_log
    
    ID fields: policy_id, evaluated_at
    """

    action: str
    allowed: bool = Field(default=False)
    evaluated_at: datetime = Field(default_factory=datetime.utcnow)
    policy_id: str = Field(description="Reference: policy.id")
    reason: Optional[str] = None
    resource: str
    user_id: Optional[str] = Field(default=None, description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


