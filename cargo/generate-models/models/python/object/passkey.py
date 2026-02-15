# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class PasskeyCredential(BaseModel):
    """
    passkey_credential
    
    ID fields: credential_id
    """

    aaguid: Optional[str] = None
    backed_up: bool = Field(default=False)
    counter: int = Field(default=0)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    credential_id: str
    device_type: Optional[str] = None
    friendly_name: Optional[str] = None
    is_active: bool = Field(default=True)
    last_used_at: Optional[datetime] = None
    public_key_fingerprint: Optional[str] = None
    transports: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyChallenge(BaseModel):
    """
    passkey_challenge
    
    ID fields: challenge
    """

    challenge: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    expires_at: datetime
    rp_id: str
    type: str = Field(default="registration")
    used: bool = Field(default=False)
    user_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class PasskeyAuthLog(BaseModel):
    """
    passkey_auth_log
    
    ID fields: credential_id, authenticated_at
    """

    authenticated_at: datetime = Field(default_factory=datetime.utcnow)
    credential_id: str
    ip_address: Optional[str] = None
    success: bool = Field(default=True)
    user_agent: Optional[str] = None
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


