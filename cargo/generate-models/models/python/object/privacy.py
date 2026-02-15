# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class PrivacyConsent(BaseModel):
    """
    privacy_consent
    
    ID fields: user_id, consent_type
    """

    consent_type: str = Field(default="necessary")
    granted: bool = Field(default=False)
    granted_at: Optional[datetime] = None
    ip_address: Optional[str] = None
    revoked_at: Optional[datetime] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    user_agent: Optional[str] = None
    user_id: str = Field(description="Reference: user.id")
    version: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PrivacyConsentHistory(BaseModel):
    """
    privacy_consent_history
    
    ID fields: user_id, consent_type, created_at
    """

    action: str = Field(default="granted")
    consent_type: str = Field(default="necessary")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    ip_address: Optional[str] = None
    user_id: str = Field(description="Reference: user.id")
    version: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class PrivacyDataExport(BaseModel):
    """
    privacy_data_export
    
    ID fields: user_id, created_at
    """

    completed_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    download_url: Optional[str] = None
    expires_at: Optional[datetime] = None
    format: str = Field(default="json")
    status: str = Field(default="pending")
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class PrivacyDataDeletion(BaseModel):
    """
    privacy_data_deletion
    
    ID fields: user_id, created_at
    """

    cancelled_at: Optional[datetime] = None
    completed_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    delete_type: str = Field(default="soft")
    reason: str
    scheduled_at: Optional[datetime] = None
    status: str = Field(default="pending")
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class PrivacyDataAccessLog(BaseModel):
    """
    privacy_data_access_log
    
    ID fields: user_id, accessed_at
    """

    access_type: str = Field(default="read")
    accessed_at: datetime = Field(default_factory=datetime.utcnow)
    accessor_id: str
    purpose: Optional[str] = None
    resource: str
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class PrivacyPolicyVersion(BaseModel):
    """
    privacy_policy_version
    
    ID fields: version
    """

    content: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    effective_date: datetime
    is_active: bool = Field(default=False)
    policy_type: str = Field(default="privacy")
    title: str
    version: str
    class Config:
        from_attributes = True
        populate_by_name = True


