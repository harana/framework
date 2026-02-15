# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class ApprovalRequest(BaseModel):
    """
    approval_request
    
    ID fields: id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    due_date: Optional[datetime] = None
    metadata: Optional[str] = None
    priority: str = Field(default="normal")
    request_type: str
    requester_id: str = Field(description="Reference: user.id")
    resource_id: str
    resource_type: str
    status: str = Field(default="pending")
    title: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class ApprovalApprover(BaseModel):
    """
    approval_approver
    
    ID fields: request_id, approver_id
    """

    approver_id: str = Field(description="Reference: user.id")
    comment: Optional[str] = None
    decided_at: Optional[datetime] = None
    is_required: bool = Field(default=True)
    reason: Optional[str] = None
    request_id: str = Field(description="Reference: approval_request.id")
    status: str = Field(default="pending")
    class Config:
        from_attributes = True
        populate_by_name = True


class ApprovalHistory(BaseModel):
    """
    approval_history
    
    ID fields: request_id, created_at
    """

    action: str = Field(default="created")
    actor_id: Optional[str] = Field(default=None, description="Reference: user.id")
    comment: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    request_id: str = Field(description="Reference: approval_request.id")
    class Config:
        from_attributes = True
        populate_by_name = True


