# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class UserPermission(BaseModel):
    """
    user_permission
    
    ID fields: user_id, resource, action
    """

    action: str
    conditions: Optional[str] = None
    expires_at: Optional[datetime] = None
    granted_at: datetime = Field(default_factory=datetime.utcnow)
    granted_by: Optional[str] = Field(default=None, description="Reference: user.id")
    resource: str
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class EffectivePermission(BaseModel):
    """
    effective_permission
    
    ID fields: user_id, resource, action
    """

    action: str
    allowed: bool = Field(default=False)
    resource: str
    source_id: Optional[str] = None
    source_type: str = Field(default="direct")
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


