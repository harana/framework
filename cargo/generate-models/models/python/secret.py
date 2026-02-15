# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Secret(BaseModel):
    """
    secret
    
    ID fields: key
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    is_rotatable: bool = Field(default=False)
    key: str
    last_rotated_at: Optional[datetime] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    value: str
    version: int = Field(default=1)
    class Config:
        from_attributes = True
        populate_by_name = True


class SecretVersion(BaseModel):
    """
    secret_version
    
    ID fields: secret_id, version
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    created_by: Optional[str] = Field(default=None, description="Reference: user.id")
    is_active: bool = Field(default=True)
    secret_id: str = Field(description="Reference: secret.id")
    value: str
    version: int
    class Config:
        from_attributes = True
        populate_by_name = True


class SecretAccessLog(BaseModel):
    """
    secret_access_log
    
    ID fields: secret_id, accessed_at, user_id
    """

    accessed_at: datetime = Field(default_factory=datetime.utcnow)
    method: str = Field(default="read")
    ip_address: Optional[str] = None
    secret_id: str = Field(description="Reference: secret.id")
    user_id: Optional[str] = Field(default=None, description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class SecretInfo(BaseModel):
    """
    secret_info
    
    ID fields: id
    """

    name: str
    description: str
    created_at: datetime
    updated_at: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


